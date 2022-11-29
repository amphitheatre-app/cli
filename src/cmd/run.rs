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
use errors::{Ok, Result};

use crate::ops;

/// Run a pipeline, build & deploy once
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// Show the build logs and output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    cleanup: bool,

    /// Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!
    #[arg(long, action= clap::ArgAction::SetTrue)]
    force: bool,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]")]
    profile: Option<Vec<String>>,

    /// Stream logs from deployed objects
    #[arg(long, action= clap::ArgAction::SetTrue)]
    tail: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        ops::run()
    }
}
