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

use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::schema::EitherCharacter;
use ignore::WalkBuilder;
use tar::Builder;
use tracing::{debug, info};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils;

pub async fn dev(ctx: Arc<Context>) -> Result<()> {
    // Create playbook from this Character
    let path = Finder::new().find().map_err(Errors::NotFoundManifest)?;
    let content = utils::read_manifest(&path)?;

    let payload = PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: EitherCharacter::Manifest(content),
        live: true,
    };
    debug!("{:#?}", payload);

    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    let playbook = client.playbooks().create(payload).map_err(Errors::ClientError)?;
    info!("The playbook was created and deployed successfully!");
    debug!("{:#?}", playbook);

    // let bytes = archive(path.parent().unwrap().to_str().unwrap()).unwrap();
    // let manifest: Manifest = toml::from_str(&content).map_err(Errors::InvalidManifest)?;
    // client
    //     .actors()
    //     .sync(playbook.id, manifest.name, bytes)
    //     .map_err(Errors::ClientError)?;

    Ok(())
}

#[allow(dead_code)]
/// Archive the given directory into a tarball and return the bytes.
fn archive(path: &str) -> Result<Vec<u8>> {
    debug!("The given path is {:?}", path);
    let mut tar = Builder::new(Vec::new());

    let base = std::path::Path::new(path);
    for entry in WalkBuilder::new(path).build() {
        let entry = entry.map_err(Errors::WalkError)?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        debug!("append path: {:?}, {:?}", path, path.to_path_buf());
        let name = path.strip_prefix(base).map_err(Errors::FailedStripPrefix)?;
        debug!("Striped path is {:?}", name);
        tar.append_path(name).map_err(Errors::FailedAppendPath)?;
    }

    tar.into_inner().map_err(Errors::FailedFinishTar)
}
