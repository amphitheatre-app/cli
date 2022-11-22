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

// use clap::{Arg, ArgMatches, Command};
// use errors::Result;

// pub fn build() -> Command<'static> {
//     Command::new("fix")
//         .about("Update old configuration to a newer schema version")
//         .args(&[
//             Arg::new("profile").short('p').long("profile").default_value("[]").help("Activate profiles by name (prefixed with `-` to disable a profile)"),
//             Arg::new("output").short('o').long("output").takes_value(true).help("File to write the changed config (instead of standard output)"),
//             Arg::new("overwrite").long("overwrite").takes_value(false).help("Overwrite original config with fixed config"),
//             Arg::new("version").long("version").default_value("amp/v1").help("Target schema version to upgrade to")
//         ])
//         .after_help(super::AFTER_HELP_STRING)
// }

// pub fn execute(args: &ArgMatches) -> Result<()> {
//     todo!()
// }

use clap::Args;
use errors::Result;

/// Update old configuration to a newer schema version
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// File to write the changed config (instead of standard output)
    #[arg(short, long)]
    output: Option<String>,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]")]
    profile: Option<Vec<String>>,

    /// Overwrite original config with fixed config
    #[arg(long, action = clap::ArgAction::SetTrue)]
    overwrite: bool,

    /// Target schema version to upgrade to
    #[arg(long, default_value = "v1")]
    version: Option<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
