// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;
use std::sync::Arc;

use amp_client::playbooks::{PlaybookPayload, Playbooks};
use amp_common::filesystem::Finder;
use amp_common::resource::{CharacterSpec, PlaybookSpec, Preface};
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::ops::{cleaner, logger, watcher};
use crate::utils;

/// The options for the pipeline.
pub struct Options {
    /// Delete deployments after dev or debug mode is interrupted
    pub cleanup: bool,
    /// Stream logs from deployed objects
    pub tail: bool,
    /// Whether this character is live or not
    pub live: bool,
    /// Exit after one sync with live mode
    pub once: bool,
}

/// Create a playbook from the remote git repository.
pub async fn pull(ctx: &Context, repository: &str) -> Result<PlaybookSpec> {
    create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::repository(repository),
        },
    )
    .await
}

/// Create a playbook from the remote registry.
pub async fn fetch(ctx: &Context, name: &str) -> Result<PlaybookSpec> {
    create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::registry(name, "hub", "latest"),
        },
    )
    .await
}

/// Create a playbook from the local manifest file.
pub async fn load(ctx: &Context, filename: &Option<PathBuf>, once: bool) -> Result<PlaybookSpec> {
    // load the character from the local character manifest.
    let path = &filename.clone().unwrap_or(Finder::new().find().map_err(Errors::NotFoundManifest)?);
    ctx.session.load(path).await?;

    let manifest = ctx.session.character.read().await.clone().unwrap();
    let character = CharacterSpec { live: true, once, ..CharacterSpec::from(&manifest) };

    create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::manifest(&character),
        },
    )
    .await
}

/// Create a playbook from the given payload.
pub async fn create(client: Playbooks<'_>, payload: PlaybookPayload) -> Result<PlaybookSpec> {
    let playbook = client.create(payload).await.map_err(Errors::FailedCreatePlaybook)?;

    info!("The playbook begins to create...");
    debug!("The created playbook is:\n {:#?}", playbook);

    Ok(playbook)
}

/// Run a pipeline.
pub async fn run(ctx: &Arc<Context>, playbook: PlaybookSpec, options: Options) -> Result<()> {
    // wait playbook resolve finished.
    sleep(Duration::from_secs(10)).await;

    let playbook = ctx.client.playbooks().get(&playbook.id).await.map_err(Errors::ClientError)?;
    ctx.session.playbook.write().await.replace(playbook.clone());

    let pid = Arc::new(playbook.id.clone());
    let name = Arc::new(lead_name(&playbook).ok_or(Errors::InvalidCharacter)?);

    // Initial sync the full sources into the server.
    if options.live {
        info!("Syncing the full sources into the server...");
        let workspace = ctx.session.workspace.read().await.clone().unwrap();
        utils::upload(&ctx.client.actors(), &pid, &name, &workspace).await?;
    }

    // Watch file changes and sync the changed files.
    if !options.once {
        let client1 = ctx.client.clone();
        let pid1 = pid.clone();
        let name1 = name.clone();
        let workspace = ctx.session.workspace.read().await.clone().unwrap();

        tokio::spawn(async move {
            if let Err(err) = watcher::watch(&workspace, &client1, &pid1, &name1).await {
                error!("The watcher is stopped: {:?}", err);
            }
        });
    }

    info!("The playbook is running...");

    // Receive the log stream from the server.
    if options.tail {
        if let Err(err) = logger::tail(&ctx.client, &pid, &name).await {
            error!("The log stream is stopped: {:?}", err);
        }
    }

    // Cleanup the playbook if cleanup is enabled.
    if options.cleanup {
        if let Err(err) = cleaner::try_cleanup_playbook(ctx).await {
            error!("Failed to cleanup playbook: {:?}", err);
        }
    }

    Ok(())
}

/// get lead character name based on preface type.
fn lead_name(playbook: &PlaybookSpec) -> Option<String> {
    if playbook.preface.registry.is_some() || playbook.preface.manifest.is_some() {
        return playbook.preface.name.clone();
    }

    if let Some(repo) = &playbook.preface.repository {
        if let Some(characters) = &playbook.characters {
            return characters
                .iter()
                .find(|x: &&CharacterSpec| x.meta.repository.eq(&repo.repo))
                .map(|x: &CharacterSpec| x.meta.name.clone());
        }
    }

    None
}
