// Copyright 2023 The Amphitheatre Authors.
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

use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::resource::{CharacterSpec, Preface};
use tracing::{error, info};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::ops::{logger, watcher};
use crate::utils;

pub async fn dev(ctx: Arc<Context>, options: &crate::cmd::dev::Cli) -> Result<()> {
    // Create playbook from local manifest file
    let path = match &options.filename {
        Some(filename) => PathBuf::from(filename),
        None => Finder::new().find().map_err(Errors::NotFoundManifest)?,
    };
    let workspace = Arc::new(path.parent().unwrap().to_path_buf());

    let manifest = utils::read_manifest(&path)?;
    ctx.session.character.write().await.replace(manifest.clone());

    let mut character = CharacterSpec::from(manifest.clone());
    character.live = true;

    let playbook = utils::create(
        ctx.client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::manifest(&character),
        },
    )?;
    ctx.session.playbook.write().await.replace(playbook.clone());

    let pid = Arc::new(playbook.id);
    let name = Arc::new(manifest.meta.name);

    // Initial sync the full sources into the server.
    info!("Syncing the full sources into the server...");
    utils::upload(&ctx.client.actors(), &pid, &name, &workspace)?;

    // Watch file changes and sync the changed files.
    let client1 = ctx.client.clone();
    let pid1 = pid.clone();
    let name1 = name.clone();
    let workspace1 = workspace.clone();

    tokio::spawn(async move {
        if let Err(err) = watcher::watch(&workspace1, &client1, &pid1, &name1).await {
            error!("The watcher is stopped: {:?}", err);
        }
    });

    // Receive the log stream from the server.
    if let Err(err) = logger::tail(&ctx.client, &pid, &name).await {
        error!("The log stream is stopped: {:?}", err);
    }

    Ok(())
}
