// Copyrgiht 2023 The Amphitheatre Authors.
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

use clap::Args;

use crate::errors::Result;

/// Run a pipeline in debug mode
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true", env = "AMP_ASSUME_YES")]
    assume_yes: bool,

    /// When set to false, builds wait for API request instead of running automatically
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_AUTO_BUILD")]
    auto_build: bool,

    /// If true, amp will try to create a config for the user's run if it doesn't find one
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_AUTO_CREATE_CONFIG")]
    auto_create_config: bool,

    /// When set to false, deploys wait for API request instead of running automatically
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_AUTO_DEPLOY")]
    auto_deploy: bool,

    /// When set to false, syncs wait for API request instead of running automatically
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_AUTO_SYNC")]
    auto_sync: bool,

    /// Delete deployments after dev or debug mode is interrupted
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_CLEANUP")]
    cleanup: bool,

    /// File for global configurations
    #[arg(short, long, default_value = "$~/.amp/config", env = "AMP_CONFIG")]
    config: Option<String>,

    /// Path or URL to the Amphitheatre config file
    #[arg(short, long, default_value = ".amp.toml", env = "AMP_FILENAME")]
    filename: Option<String>,

    /// Recreate Kubernetes resources if necessary for deployment,
    /// warning: might cause downtime!
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_FORCE")]
    force: bool,

    /// Run `status-check` iteratively after each deploy step,
    /// instead of all-together at the end of all deploys (default)
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_ITERATIVE_STATUS_CHECK")]
    iterative_status_check: bool,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]", env = "AMP_PROFILE")]
    profile: Option<Vec<String>>,

    /// Priority sorted order of debugger protocols to support
    #[arg(long, default_value = "[]", env = "AMP_PROTOCOLS")]
    protocols: Option<String>,

    /// Wait for deployed resources to stabilize
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_STATUS_CHECK")]
    status_check: bool,

    /// Stream logs from deployed objects
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_TAIL")]
    tail: bool,

    /// How is change detection triggered? (polling, notify, or manual)
    #[arg(long, default_value = "notify", env = "AMP_TRIGGER")]
    trigger: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
