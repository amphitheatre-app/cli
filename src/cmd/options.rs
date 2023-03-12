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

use clap::{Args, CommandFactory};

use crate::cmd::cli::Cli as RootCli;
use crate::errors::Result;

/// Output a list of global command-line options (applies to all commands)
#[derive(Args, Debug)]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let cmd = RootCli::command();
        for arg in cmd.get_arguments().filter(|a| a.is_global_set()) {
            println!(
                "{}\t{:<20}\t{:<2}",
                arg.get_short().map(|v| format!("-{v}")).unwrap_or_default(),
                arg.get_long().map(|v| format!("--{v}")).unwrap_or_default(),
                arg.get_help().unwrap_or_default()
            );
        }

        Ok(())
    }
}
