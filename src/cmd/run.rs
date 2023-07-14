// Copyright 2023 The Amphitheatre Authors.
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

use std::sync::Arc;

use clap::Args;

use crate::context::Context;
use crate::errors::Result;
use crate::ops;

/// Run a pipeline, build & deploy once
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true", env = "AMP_ASSUME_YES")]
    assume_yes: bool,

    /// Delete deployments after dev or debug mode is interrupted
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_CLEANUP")]
    cleanup: bool,

    /// Path or URL to the Amphitheatre config file
    #[arg(short, long, env = "AMP_FILENAME")]
    pub filename: Option<String>,

    /// Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_FORCE")]
    force: bool,

    /// The URL of the remote git repository for your character where you want to run
    #[arg(long, env = "AMP_GIT")]
    pub git: Option<String>,

    /// The name of the character on the cluster you want to run on
    #[arg(long, env = "AMP_NAME")]
    pub name: Option<String>,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]", env = "AMP_PROFILE")]
    profile: Option<Vec<String>>,

    /// Stream logs from deployed objects
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_TAIL")]
    tail: bool,
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        // Handle signal.
        ctrlc::set_handler(|| {
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        ops::run(ctx, self).await
    }
}
