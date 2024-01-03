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

use std::{path::PathBuf, sync::Arc};

use amp_client::{actors::Actor, client::Client, playbooks::Playbook};
use amp_common::{
    config::{Cluster, Configuration},
    filesystem::Finder,
    schema::Character,
};
use tokio::sync::RwLock;
use tracing::debug;

use crate::errors::{Errors, Result};

/// Session holds the current session state
#[derive(Default, Debug)]
pub struct Session {
    pub workspace: RwLock<Option<PathBuf>>,
    pub character: RwLock<Option<Character>>,
    pub playbook: RwLock<Option<Playbook>>,
    pub actor: RwLock<Option<Actor>>,
}

impl Session {
    pub fn init() -> Session {
        // Try to load the character from the current or parent directory.
        let (workspace, character) = try_load_character()
            .map(|(workspace, character)| (Some(workspace), Some(character)))
            .unwrap_or((None, None));

        Session {
            workspace: RwLock::new(workspace),
            character: RwLock::new(character),
            playbook: RwLock::new(None),
            actor: RwLock::new(None),
        }
    }

    /// Load the character from the specified file.
    pub async fn load(&self, filename: &str) -> Result<()> {
        let path = PathBuf::from(filename);
        let workspace = path.parent().unwrap().to_path_buf();
        let character = Character::load(&path).map_err(|e| Errors::FailedLoadManifest(e.to_string()))?;

        self.workspace.write().await.replace(workspace);
        self.character.write().await.replace(character);

        Ok(())
    }
}

/// Try to load the character from the current directory
fn try_load_character() -> Option<(PathBuf, Character)> {
    let path = Finder::new().find();
    if let Err(err) = path {
        debug!("Not found character in current or parent directories: {}", err);
        return None;
    }

    let path = path.unwrap();
    let workspace = path.parent().unwrap().to_path_buf();
    let character = Character::load(&path);
    if let Err(err) = character {
        debug!("Failed to read character manifest: {}", err);
        return None;
    }

    Some((workspace, character.unwrap()))
}

/// Context holds the current context state
pub struct Context {
    pub configuration: RwLock<Configuration>,
    pub cluster: RwLock<Cluster>,
    pub session: Session,
    pub client: Arc<Client>,
}

impl Context {
    /// Initialize a new context
    pub fn init() -> Result<Context> {
        let path = Configuration::path().map_err(Errors::InvalidConfigPath)?;
        let configuration = Configuration::load(path).map_err(Errors::FailedLoadConfiguration)?;
        let cluster = get_context(&configuration)?;
        let client = Client::new(&format!("{}/v1", &cluster.server), cluster.token.clone());

        Ok(Context {
            configuration: RwLock::new(configuration),
            cluster: RwLock::new(cluster),
            session: Session::init(),
            client: Arc::new(client),
        })
    }
}

/// Get the current context from the configuration
fn get_context(configuration: &Configuration) -> Result<Cluster> {
    if let Some(context) = &configuration.context {
        if let Some((_, current)) = context.current() {
            return Ok(current.to_owned());
        }
    }

    Err(Errors::NotFoundCurrentContext)
}
