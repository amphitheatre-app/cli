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

use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::resource::{CharacterSpec, Preface};
use tracing::info;

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils;

pub async fn run(ctx: Arc<Context>, options: &crate::cmd::run::Cli) -> Result<()> {
    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    // Create playbook from cluster, return if success
    if let Some(repository) = &options.git {
        return utils::create(
            client.playbooks(),
            PlaybookPayload {
                title: "Untitled".to_string(),
                description: "".to_string(),
                preface: Preface::repository(repository),
            },
        )
        .map(|_| ());
    }

    // Create playbook from registry, return if success
    if let Some(name) = &options.name {
        return utils::create(
            client.playbooks(),
            PlaybookPayload {
                title: "Untitled".to_string(),
                description: "".to_string(),
                preface: Preface::registry(name, "hub", "latest"),
            },
        )
        .map(|_| ());
    }

    // Create playbook from local manifest file
    let path = match &options.filename {
        Some(filename) => PathBuf::from(filename),
        None => Finder::new().find().map_err(Errors::NotFoundManifest)?,
    };
    let workspace = path.parent().unwrap();
    let manifest = utils::read_manifest(&path)?;

    let playbook = utils::create(
        client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::manifest(&CharacterSpec::from(manifest.clone())),
        },
    )?;

    // Sync the full sources into the server for build.
    info!("Syncing the full sources into the server...");
    utils::upload(&client.actors(), &playbook.id, &manifest.meta.name, workspace)?;

    Ok(())
}
