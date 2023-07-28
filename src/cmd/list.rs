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

use std::sync::Arc;

use amp_client::client::Client;
use amp_client::playbooks::Playbook;
use clap::Args;
use tabled::settings::Style;
use tabled::Tabled;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// List all running instances
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let context = ctx.context().await?;
        let client = Client::new(&format!("{}/v1", &context.server), context.token);
        let playbooks = client.playbooks().list(None).map_err(Errors::ClientError)?;

        if playbooks.is_empty() {
            println!("No playbooks found");
            return Ok(());
        }

        // Print playbooks as table
        let table: Vec<PlaybookTable> = playbooks.iter().map(|p| PlaybookTable::from(p)).collect();
        println!("{}", tabled::Table::new(table).with(Style::modern()));

        Ok(())
    }
}

#[derive(Tabled)]
struct PlaybookTable {
    id: String,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl From<&Playbook> for PlaybookTable {
    fn from(value: &Playbook) -> Self {
        Self {
            id: value.id.clone(),
            title: value.title.clone(),
            description: value.description.clone(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
        }
    }
}
