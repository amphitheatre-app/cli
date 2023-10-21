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
use amp_common::resource::{CharacterSpec, Preface};
use amp_common::sync::{self, EventKinds, Synchronization};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use notify::event::RemoveKind;
use notify::EventKind::Remove;
use notify::RecursiveMode::Recursive;
use notify::{Event, RecommendedWatcher, Watcher};
use tracing::{debug, error, info, trace, warn};

use crate::context::Context;
use crate::errors::{Errors, Result};
use crate::utils;

pub async fn dev(ctx: Arc<Context>) -> Result<()> {
    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    // Create playbook from local manifest file
    let path = Finder::new().find().map_err(Errors::NotFoundManifest)?;
    let workspace = path.parent().unwrap();
    let manifest = utils::read_manifest(&path)?;

    let mut character = CharacterSpec::from(manifest.clone());
    character.live = true;

    let playbook = utils::create(
        client.playbooks(),
        PlaybookPayload {
            title: "Untitled".to_string(),
            description: "".to_string(),
            preface: Preface::manifest(&character),
        },
    )?;

    // Continuous Synchronize file changes.
    // first time, we need to sync all files.
    // and then, we need to sync only changed files.
    let actors = client.actors();

    // Initial sync the full sources into the server.
    info!("Syncing the full sources into the server...");
    utils::upload(&actors, &playbook.id, &manifest.meta.name, workspace)?;

    // Watch file changes and sync the changed files.
    let (tx, rx) = std::sync::mpsc::channel();
    // We listen to the file changes giving Notify
    // a function that will get called when events happen.
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).map_err(Errors::FailedCreateWatcher)?;
    watcher.watch(workspace, Recursive).map_err(Errors::FailedWatchDirectory)?;

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

        handle(&actors, &playbook.id, &manifest.meta.name, workspace, event)?;
    }

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
        paths.push(utils::strip(base, &path)?);
    }

    let mut req = Synchronization { kind: kind.clone(), paths: vec![], attributes: None, payload: None };

    // Because the file or directory was removed yet, we can't get the file type.
    // so we determine the file type by original event kind.
    if kind == EventKinds::Remove {
        let is_dir = event.kind == Remove(RemoveKind::Folder);
        req.paths = paths.iter().map(|(_, b)| format_path(b, is_dir)).collect();
    } else {
        req.paths = paths.iter().map(|(a, b)| format_path(b, a.is_dir())).collect();
    }

    if kind == EventKinds::Modify {
        req.payload = Some(utils::archive(&paths)?);
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
