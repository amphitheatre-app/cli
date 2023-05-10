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

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::context::Context;
use crate::errors::Result;

pub const AFTER_HELP_STRING: &str =
    "Use \"amp options\" for a list of global command-line options (applies to all commands).";
pub const DEFAULT_CONFIG_FILEPATH: &str = "~/.config/amphitheatre/config.toml";

/// Amphitheatre's offcial command line tool
#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    disable_help_subcommand = false,
    after_help = AFTER_HELP_STRING,
)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,

    /// File for global configurations
    #[arg(short, long, default_value = DEFAULT_CONFIG_FILEPATH, env = "AMP_CONFIG", global=true)]
    config: Option<String>,

    /// Allow user prompts for more information
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_INTERACTIVE", global=true)]
    interactive: bool,

    /// Print timestamps in logs
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_TIMESTAMPS", global=true)]
    timestamps: bool,

    /// Check for a more recent version of Amphitheatre
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_UPDATE_CHECK", global=true)]
    update_check: bool,

    /// Log level: one of [panic fatal error warning info debug trace]
    #[arg(long, default_value = "warning", env = "AMP_VERBOSITY", global = true)]
    verbosity: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Clean(super::clean::Cli),
    Context(super::context::cli::Cli),
    Completion(super::completion::Cli),
    Config(super::config::cli::Cli),
    Debug(super::debug::Cli),
    Deploy(super::deploy::Cli),
    Dev(super::dev::Cli),
    Diagnose(super::diagnose::Cli),
    Init(super::init::Cli),
    List(super::list::Cli),
    Options(super::options::Cli),
    Render(super::render::Cli),
    Run(super::run::Cli),
    Test(super::test::Cli),
    Version(super::version::Cli),
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        match &self.command {
            Commands::Clean(cli) => cli.exec(ctx).await,
            Commands::Context(cli) => cli.exec(ctx).await,
            Commands::Completion(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(ctx).await,
            Commands::Debug(cli) => cli.exec(ctx).await,
            Commands::Deploy(cli) => cli.exec(ctx).await,
            Commands::Dev(cli) => cli.exec(ctx).await,
            Commands::Diagnose(cli) => cli.exec(ctx).await,
            Commands::Init(cli) => cli.exec(ctx).await,
            Commands::List(cli) => cli.exec(ctx).await,
            Commands::Options(cli) => cli.exec(),
            Commands::Render(cli) => cli.exec(ctx).await,
            Commands::Run(cli) => cli.exec(ctx).await,
            Commands::Test(cli) => cli.exec(ctx).await,
            Commands::Version(cli) => cli.exec(),
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
