// Copyright 2022 The Amphitheatre Authors.
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
use errors::Result;

use crate::ops;

/// Run a pipeline, build & deploy once
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true")]
    assume_yes: bool,

    /// If true, amp will try to create a config for the user's run if it doesn't find one
    #[arg(long, action= clap::ArgAction::SetTrue)]
    auto_create_config: bool,

    /// Show the build logs and output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    cleanup: bool,

    /// File for global configurations
    #[arg(short, long, default_value = "~/.amp/config")]
    config: Option<String>,

    /// Path or URL to the Amphitheatre config file
    #[arg(short, long, default_value = ".amp.toml")]
    filename: Option<String>,

    /// Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!
    #[arg(long, action= clap::ArgAction::SetTrue)]
    force: bool,

    /// Run `status-check` iteratively after each deploy step,
    /// instead of all-together at the end of all deploys (default)
    #[arg(long, action= clap::ArgAction::SetTrue)]
    iterative_status_check: bool,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]")]
    profile: Option<Vec<String>>,

    /// Wait for deployed resources to stabilize
    #[arg(long, action= clap::ArgAction::SetTrue)]
    status_check: bool,

    /// Stream logs from deployed objects
    #[arg(long, action= clap::ArgAction::SetTrue)]
    tail: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        ops::run()
    }
}
