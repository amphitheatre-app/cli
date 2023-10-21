// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};

use amp_client::{
    actors::Actors,
    playbooks::{Playbook, PlaybookPayload, Playbooks},
};
use amp_common::{
    schema::Character,
    sync::{EventKinds, Synchronization},
};
use ignore::WalkBuilder;
use tar::Builder;
use tracing::{debug, info};

use crate::errors::{Errors, Result};

/// Read manifest file, validate and return content
pub fn read_manifest<P: AsRef<Path>>(path: P) -> Result<Character> {
    let content = std::fs::read_to_string(path).map_err(|e| Errors::FailedLoadManifest(e.to_string()))?;
    toml::from_str::<Character>(&content).map_err(Errors::InvalidManifest)
}

/// Upload the given directory to the server.
pub fn upload(client: &Actors, pid: &str, name: &str, workspace: &Path) -> Result<()> {
    let mut paths: Vec<(PathBuf, PathBuf)> = vec![];

    let base = workspace;
    for entry in WalkBuilder::new(workspace).build() {
        let entry = entry.map_err(Errors::WalkError)?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        paths.push(strip(base, path)?);
    }

    let payload = archive(&paths)?;
    let req = Synchronization { kind: EventKinds::Overwrite, paths: vec![], attributes: None, payload: Some(payload) };
    client.sync(pid, name, req).map_err(Errors::ClientError)?;

    Ok(())
}

/// Archive the given directory into a tarball and return the bytes.
pub fn archive(paths: &Vec<(PathBuf, PathBuf)>) -> Result<Vec<u8>> {
    debug!("The given path for archive is {:?}", paths);
    let mut tar = Builder::new(Vec::new());
    for (path, name) in paths {
        tar.append_path_with_name(path, name).map_err(Errors::FailedAppendPath)?;
    }
    tar.into_inner().map_err(Errors::FailedFinishTar)
}

/// Strip the given base path from the given path.
#[inline]
pub fn strip(base: &Path, path: &Path) -> Result<(PathBuf, PathBuf)> {
    let striped_path = path.strip_prefix(base).map_err(Errors::FailedStripPrefix)?;
    debug!("the full path and striped path is: {:?}, {:?}", path, striped_path);
    Ok((path.to_path_buf(), striped_path.to_path_buf()))
}

pub fn create(client: Playbooks, payload: PlaybookPayload) -> Result<Playbook> {
    let playbook = client.create(payload).map_err(Errors::FailedCreatePlaybook)?;
    info!("The playbook was created successfully!");
    debug!("{:#?}", playbook);

    Ok(playbook)
}
