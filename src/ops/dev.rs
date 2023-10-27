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

use std::sync::Arc;

use amp_client::playbooks::PlaybookPayload;
use amp_common::resource::{CharacterSpec, Preface};
use tracing::{error, info};

use crate::context::Context;
use crate::errors::Result;
use crate::ops::{logger, watcher};
use crate::utils;

pub async fn dev(ctx: Arc<Context>, options: &crate::cmd::dev::Cli) -> Result<()> {
    // Read the character manifest from the specified file.
    if let Some(filename) = &options.filename {
        ctx.session.load(filename).await?;
    }

    let manifest = ctx.session.character.read().await.clone().unwrap();
    let character = CharacterSpec { live: true, ..CharacterSpec::from(&manifest) };

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
    let workspace = ctx.session.workspace.read().await.clone().unwrap();
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
