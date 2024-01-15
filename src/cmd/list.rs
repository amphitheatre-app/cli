// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use amp_common::resource::PlaybookSpec;
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
        let playbooks = ctx.client.playbooks().list(None).map_err(Errors::ClientError)?;

        if playbooks.is_empty() {
            println!("No playbooks found");
            return Ok(());
        }

        // Print playbooks as table
        let table: Vec<PlaybookTable> = playbooks.iter().map(PlaybookTable::from).collect();
        println!("{}", tabled::Table::new(table).with(Style::modern()));

        Ok(())
    }
}

#[derive(Tabled)]
struct PlaybookTable {
    id: String,
    title: String,
    description: String,
}

impl From<&PlaybookSpec> for PlaybookTable {
    fn from(value: &PlaybookSpec) -> Self {
        Self {
            id: value.id().to_string(),
            title: value.title.clone(),
            description: value.description.clone().unwrap_or_default(),
        }
    }
}
