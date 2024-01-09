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

use std::sync::Arc;

use amp_client::playbooks::{Playbook, PlaybookPayload, Playbooks};
use amp_common::resource::{CharacterSpec, Preface};
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
pub fn pull(ctx: &Context, repository: &str) -> Result<Playbook> {
    create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::repository(repository),
        },
    )
}

/// Create a playbook from the remote registry.
pub fn fetch(ctx: &Context, name: &str) -> Result<Playbook> {
    create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::registry(name, "hub", "latest"),
        },
    )
}

/// Create a playbook from the local manifest file.
pub async fn load(ctx: &Context, live: bool, once: bool) -> Result<Playbook> {
    let manifest = ctx.session.character.read().await.clone().unwrap();
    let character = CharacterSpec { live, once, ..CharacterSpec::from(&manifest) };

    return create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::manifest(&character),
        },
    );
}

/// Create a playbook from the given payload.
pub fn create(client: Playbooks, payload: PlaybookPayload) -> Result<Playbook> {
    let playbook = client.create(payload).map_err(Errors::FailedCreatePlaybook)?;

    info!("The playbook begins to create...");
    debug!("The created playbook is:\n {:#?}", playbook);

    Ok(playbook)
}

/// Run a pipeline.
pub async fn run(ctx: &Arc<Context>, playbook: Playbook, options: Options) -> Result<()> {
    ctx.session.playbook.write().await.replace(playbook.clone());

    let character = ctx.session.character.read().await.clone().unwrap();
    let pid = Arc::new(playbook.id);
    let name = Arc::new(character.meta.name);

    // Initial sync the full sources into the server.
    if options.live {
        info!("Syncing the full sources into the server...");
        let workspace = ctx.session.workspace.read().await.clone().unwrap();
        utils::upload(&ctx.client.actors(), &pid, &name, &workspace)?;
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
