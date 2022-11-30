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

use clap::Args;
use errors::Result;

/// Output a list of global command-line options (applies to all commands)
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// Allow user prompts for more information
    #[arg(long, action = clap::ArgAction::SetTrue)]
    interactive: bool,

    /// Print timestamps in logs
    #[arg(long, action = clap::ArgAction::SetTrue)]
    timestamps: bool,

    /// Check for a more recent version of Amphitheatre
    #[arg(long, action = clap::ArgAction::SetTrue)]
    update_check: bool,

    /// Log level: one of [panic fatal error warning info debug trace]
    #[arg(long, default_value = "warning")]
    verbosity: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
