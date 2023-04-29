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

use log::LevelFilter;
use once_cell::sync::OnceCell;

use crate::config::Config;
use crate::context::ContextConfiguration;

static CONFIG: OnceCell<Config> = OnceCell::new();
static CONTEXTS: OnceCell<ContextConfiguration> = OnceCell::new();
static VERBOSITY: OnceCell<LevelFilter> = OnceCell::new();

#[allow(dead_code)]
pub fn config() -> &'static Config {
    CONFIG.get().expect("Config is not initialized")
}

pub fn set_global_config(config: Config) {
    CONFIG.set(config).expect("Could not set config")
}

#[allow(dead_code)]
pub fn contexts() -> &'static ContextConfiguration {
    CONTEXTS.get().expect("Context configuration is not initialized")
}

pub fn set_global_contexts(config: ContextConfiguration) {
    CONTEXTS.set(config).expect("Could not set context configuration")
}

#[allow(dead_code)]
pub fn verbosity() -> &'static LevelFilter {
    VERBOSITY.get().expect("Verbosity is not initialized")
}

pub fn set_global_verbosity(verbosity: LevelFilter) {
    VERBOSITY.set(verbosity).expect("Could not set verbosity")
}
