// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::errors::Result;

/// Interact with the global Amphitheatre config file
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Find(super::find::Cli),
    List(super::list::Cli),
    Set(super::set::Cli),
    Unset(super::unset::Cli),
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        match &self.command {
            Commands::List(cli) => cli.exec(ctx).await,
            Commands::Set(cli) => cli.exec(ctx).await,
            Commands::Unset(cli) => cli.exec(ctx).await,
            Commands::Find(cli) => cli.exec(ctx).await,
        }
    }
}
