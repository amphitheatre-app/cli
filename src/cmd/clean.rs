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

use amp_client::client::Client;
use clap::Args;

use crate::context::Context;
use crate::errors::{anyhow, Result};

/// Delete any resources deployed by Amphitheatre
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// The ID of the playbook to delete
    id: String,

    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true", env = "AMP_ASSUME_YES")]
    assume_yes: bool,

    /// If true, amp will skip yes/no confirmation from the user and default to yes
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_DRY_RUN")]
    dry_run: bool,

    /// Path or URL to the Amphitheatre config file
    #[arg(short, long, default_value = ".amp.toml", env = "AMP_FILENAME")]
    filename: Option<String>,
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let context = ctx.context().await?;
        let client = Client::new(&format!("{}/v1", &context.server), context.token);

        let status = client.playbooks().delete(&self.id)?;
        if status != 200 {
            return Err(anyhow!("Failed to delete playbook"));
        }

        Ok(())
    }
}
