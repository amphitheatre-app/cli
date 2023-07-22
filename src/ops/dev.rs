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

use std::path::{Path, PathBuf};
use std::sync::Arc;

use amp_client::actors::Actors;
use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::schema::{EitherCharacter, Manifest};
use amp_common::sync::{self, EventKinds, Synchronization};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::WalkBuilder;
use notify::event::RemoveKind;
use notify::EventKind::Remove;
use notify::RecursiveMode::Recursive;
use notify::{Event, RecommendedWatcher, Watcher};
use tar::Builder;
use tracing::{debug, error, info, trace, warn};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils;

pub async fn dev(ctx: Arc<Context>) -> Result<()> {
    // Create playbook from this Character
    let path = Finder::new().find().map_err(Errors::NotFoundManifest)?;
    let workspace = path.parent().unwrap();
    let content = utils::read_manifest(&path)?;

    let payload = PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: EitherCharacter::Manifest(content.clone()),
        live: true,
    };
    debug!("{:#?}", payload);

    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    let playbook = client.playbooks().create(payload).map_err(Errors::ClientError)?;
    info!("The playbook was created and deployed successfully!");
    debug!("{:#?}", playbook);

    // Continuous Synchornize file changes.
    // first time, we need to sync all files.
    // and then, we need to sync only changed files.
    let actors = client.actors();
    let manifest: Manifest = toml::from_str(&content).map_err(Errors::InvalidManifest)?;

    // Initial sync the full sources into the server.
    info!("Syncing the full sources into the server...");
    upload(&actors, &playbook.id, &manifest.name, workspace)?;

    // Watch file changes and sync the changed files.
    let (tx, rx) = std::sync::mpsc::channel();
    // We listen to the file changes giving Notify
    // a function that will get called when events happen.
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).map_err(Errors::FailedCreateWatcher)?;
    watcher
        .watch(workspace, Recursive)
        .map_err(Errors::FailedWatchDirectory)?;

    let mut builder = GitignoreBuilder::new(workspace);
    builder.add(".gitignore");
    let matcher = builder.build().unwrap();

    for event in rx {
        if let Err(err) = event {
            error!("Got a notify error: {err:?}");
            continue;
        }
        let event = event.unwrap();
        if is_ignored(&matcher, workspace, &event.paths)? {
            continue;
        }

        handle(&actors, &playbook.id, &manifest.name, workspace, event)?;
    }

    Ok(())
}

fn upload(client: &Actors, pid: &str, name: &str, workspace: &Path) -> Result<()> {
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

    let pyaload = archive(&paths).unwrap();
    let req = Synchronization {
        kind: EventKinds::Override,
        paths: vec![],
        attributes: None,
        payload: Some(pyaload),
    };
    client.sync(pid, name, req).map_err(Errors::ClientError)?;

    Ok(())
}

fn handle(client: &Actors, pid: &str, name: &str, base: &Path, event: Event) -> Result<()> {
    trace!("Changed: {:?}", event);

    let kind = EventKinds::from(event.kind);
    if kind == EventKinds::Rename || kind == EventKinds::Other {
        warn!("Not supported event: {:?}", event);
        return Ok(());
    }

    let mut paths: Vec<(PathBuf, PathBuf)> = vec![];
    for path in event.paths {
        paths.push(strip(base, &path)?);
    }

    let mut req = Synchronization {
        kind: kind.clone(),
        paths: vec![],
        attributes: None,
        payload: None,
    };

    // Because the file or directory was removed yet, we can't get the file type.
    // so we determine the file type by original event kind.
    if kind == EventKinds::Remove {
        let is_dir = event.kind == Remove(RemoveKind::Folder);
        req.paths = paths.iter().map(|(_, b)| format_path(b, is_dir)).collect();
    } else {
        req.paths = paths.iter().map(|(a, b)| format_path(b, a.is_dir())).collect();
    }

    if kind == EventKinds::Modify {
        req.payload = Some(archive(&paths)?);
    }

    debug!("The sync request is: {:?}", req);
    client.sync(pid, name, req).map_err(Errors::ClientError)?;
    Ok(())
}

fn format_path(path: &Path, is_dir: bool) -> sync::Path {
    let path_string = path.to_str().unwrap().to_string();
    if is_dir {
        sync::Path::Directory(path_string)
    } else {
        sync::Path::File(path_string)
    }
}

/// Archive the given directory into a tarball and return the bytes.
fn archive(paths: &Vec<(PathBuf, PathBuf)>) -> Result<Vec<u8>> {
    debug!("The given path for archive is {:?}", paths);
    let mut tar = Builder::new(Vec::new());
    for (path, name) in paths {
        tar.append_path_with_name(path, name)
            .map_err(Errors::FailedAppendPath)?;
    }
    tar.into_inner().map_err(Errors::FailedFinishTar)
}

#[inline]
fn strip(base: &Path, path: &Path) -> Result<(PathBuf, PathBuf)> {
    let striped_path = path.strip_prefix(base).map_err(Errors::FailedStripPrefix)?;
    debug!("the full path and striped path is: {:?}, {:?}", path, striped_path);
    Ok((path.to_path_buf(), striped_path.to_path_buf()))
}

fn is_ignored(matcher: &Gitignore, root: &Path, paths: &Vec<PathBuf>) -> Result<bool> {
    for path in paths {
        let name = path.strip_prefix(root).map_err(Errors::FailedStripPrefix)?;
        if matcher.matched(name, false).is_ignore() {
            debug!("The file is ignored: {:?}", name);
            return Ok(true);
        }
    }
    Ok(false)
}
