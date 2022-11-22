// Copyright 2022 The Amphitheatre Authors.
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

pub const AFTER_HELP_STRING: &str =
    "Use \"amp options\" for a list of global command-line options (applies to all commands).";

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use errors::Result;

/// Amphitheatre's offcial command line tool
#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    disable_help_subcommand = false,
    after_help = AFTER_HELP_STRING
)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Apply(super::apply::Cli),
    Build(super::build::Cli),
    Clean(super::clean::Cli),
    Completion(super::completion::Cli),
    Config(super::config::cli::Cli),
    Debug(super::debug::Cli),
    Deploy(super::deploy::Cli),
    Dev(super::dev::Cli),
    Diagnose(super::diagnose::Cli),
    Fix(super::fix::Cli),
    Init(super::init::Cli),
    Options(super::options::Cli),
    Render(super::render::Cli),
    Run(super::run::Cli),
    Schema(super::schema::cli::Cli),
    Survey(super::survey::Cli),
    Test(super::test::Cli),
    Version(super::version::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Apply(cli) => cli.exec(),
            Commands::Build(cli) => cli.exec(),
            Commands::Clean(cli) => cli.exec(),
            Commands::Completion(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(),
            Commands::Debug(cli) => cli.exec(),
            Commands::Deploy(cli) => cli.exec(),
            Commands::Dev(cli) => cli.exec(),
            Commands::Diagnose(cli) => cli.exec(),
            Commands::Fix(cli) => cli.exec(),
            Commands::Init(cli) => cli.exec(),
            Commands::Options(cli) => cli.exec(),
            Commands::Render(cli) => cli.exec(),
            Commands::Run(cli) => cli.exec(),
            Commands::Schema(cli) => cli.exec(),
            Commands::Survey(cli) => cli.exec(),
            Commands::Test(cli) => cli.exec(),
            Commands::Version(cli) => cli.exec(),
        }
    }
}
