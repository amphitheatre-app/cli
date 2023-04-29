// Copyrgiht 2023 The Amphitheatre Authors.
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

use serde::{Deserialize, Serialize};

use crate::errors::{Context as _, Result};

const APP_NAME: &str = "amp";
const FILE_STEM: &str = "contexts";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Context {
    pub name: String,
    pub url: String,
    pub token: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ContextConfiguration {
    pub current: Option<Context>,
    pub contexts: Vec<Context>,
}

pub fn load() -> Result<ContextConfiguration> {
    confy::load(APP_NAME, FILE_STEM).with_context(|| "unable to load context configuration")
}

#[allow(dead_code)]
pub fn save(config: ContextConfiguration) -> Result<()> {
    confy::store(APP_NAME, FILE_STEM, config).with_context(|| "unable to save context configuration")
}
