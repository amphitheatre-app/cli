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

use std::path::Path;
use std::sync::Arc;

use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::resource::{CharacterSpec, Preface};
use tracing::{debug, info};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils;

pub async fn run(ctx: Arc<Context>, options: &crate::cmd::run::Cli) -> Result<()> {
    let payload: PlaybookPayload;

    if let Some(repository) = &options.git {
        payload = create_playbook_from_git(repository)?;
    } else if let Some(name) = &options.name {
        payload = create_playbook_from_cluster(name)?;
    } else if let Some(path) = &options.filename {
        payload = create_playbook_from_manifest(path)?;
    } else {
        let path = Finder::new().find().map_err(Errors::NotFoundManifest)?;
        payload = create_playbook_from_manifest(path)?;
    }
    debug!("{:#?}", payload);

    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    let playbook = client.playbooks().create(payload).map_err(Errors::FailedCreatePlaybook)?;
    info!("The playbook was created and deployed successfully!");
    debug!("{:#?}", playbook);

    Ok(())
}

/// Create playbook from cluster
fn create_playbook_from_cluster(name: &str) -> Result<PlaybookPayload> {
    Ok(PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: Preface::registry(name, "hub", "latest"),
    })
}

/// Create playbook from remote git repository
fn create_playbook_from_git(repository: &str) -> Result<PlaybookPayload> {
    Ok(PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: Preface::repository(repository),
    })
}

/// Create playbook from manifest file
fn create_playbook_from_manifest<P: AsRef<Path>>(path: P) -> Result<PlaybookPayload> {
    let manifest = utils::read_manifest(path)?;

    Ok(PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: Preface::manifest(&CharacterSpec::from(manifest)),
    })
}
