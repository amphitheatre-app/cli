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

use std::{path::PathBuf, sync::Arc};

use amp_client::client::Client;
use amp_common::{
    config::{Cluster, Configuration},
    resource::{ActorSpec, PlaybookSpec},
    schema::Character,
};
use tokio::sync::RwLock;

use crate::errors::{Errors, Result};

/// Session holds the current session state
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Session {
    pub workspace: RwLock<Option<PathBuf>>,
    pub character: RwLock<Option<Character>>,
    pub playbook: RwLock<Option<PlaybookSpec>>,
    pub actor: RwLock<Option<ActorSpec>>,
}

impl Session {
    /// Load the character from the specified file.
    pub async fn load(&self, path: &PathBuf) -> Result<()> {
        let workspace = path.parent().unwrap().to_path_buf();
        let character = Character::load(path).map_err(Errors::FailedLoadManifest)?;

        self.workspace.write().await.replace(workspace);
        self.character.write().await.replace(character);

        Ok(())
    }
}

/// Context holds the current context state
#[allow(dead_code)]
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
            session: Session::default(),
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
