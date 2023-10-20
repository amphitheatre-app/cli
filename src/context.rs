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

use amp_common::config::{Cluster, Configuration};
use tokio::sync::RwLock;

use crate::errors::{Errors, Result};

pub struct Context {
    pub configuration: RwLock<Configuration>,
}

impl Context {
    pub async fn init() -> Result<Context> {
        let path = Configuration::path().map_err(Errors::InvalidConfigPath)?;
        let configuration = Configuration::load(path).map_err(Errors::FailedLoadConfiguration)?;

        Ok(Context { configuration: RwLock::new(configuration) })
    }

    /// Get the current context from the configuration
    pub async fn context(&self) -> Result<Cluster> {
        let configuration = self.configuration.read().await;
        if let Some(context) = &configuration.context {
            if let Some(current) = context.current() {
                return Ok(current.to_owned());
            }
        }

        Err(Errors::NotFoundCurrentContext)
    }
}
