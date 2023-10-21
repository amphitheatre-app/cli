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

use std::fmt::Display;
use std::sync::Arc;

use amp_client::client::Client;
use clap::Args;
use inquire::Select;
use tracing::info;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Delete any resources deployed by Amphitheatre
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// The ID of the playbook to delete
    id: Option<String>,

    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true", env = "AMP_ASSUME_YES")]
    assume_yes: bool,

    /// If true, amp will skip yes/no confirmation from the user and default to yes
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_DRY_RUN")]
    dry_run: bool,

    /// If true, amp will delete all playbooks
    #[arg(long, action = clap::ArgAction::SetTrue, default_value = "false")]
    all: bool,
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let context = ctx.context().await?;
        let client = Client::new(&format!("{}/v1", &context.server), context.token);

        if let Some(id) = &self.id {
            return delete(&client, id).await;
        }

        let playbooks = client.playbooks().list(None).map_err(Errors::ClientError)?;
        if playbooks.is_empty() {
            println!("No playbooks found");
            return Ok(());
        }

        if self.all {
            if self.dry_run {
                println!("Would delete all playbooks");
                return Ok(());
            }

            for playbook in playbooks {
                delete(&client, &playbook.id).await?;
            }

            return Ok(());
        }

        // create a options list for the user to select from
        let options: Vec<OptionItem> = playbooks.iter().map(|p| OptionItem(p.id.clone(), p.title.clone())).collect();
        let answer = Select::new("Select playbook to delete: ", options).prompt().map_err(Errors::InquireError)?;
        delete(&client, answer.0.as_str()).await?;

        Ok(())
    }
}

#[derive(PartialEq)]
struct OptionItem(String, String);

impl Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}", self.0, self.1)
    }
}

async fn delete(client: &Client, id: &str) -> Result<()> {
    let status = client.playbooks().delete(id).map_err(Errors::ClientError)?;
    if status != 204 {
        return Err(Errors::FailedDeletePlaybook(id.to_string()));
    }

    info!("Deleted playbook {}", id);

    Ok(())
}
