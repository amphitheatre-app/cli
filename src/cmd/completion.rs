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

use std::io;

use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::cmd::cli::Cli as RootCli;
use crate::errors::Result;

/// Display the completion file for a given shell
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(value_enum)]
    shell: Shell,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut cmd = RootCli::command();
        let bin_name = cmd.get_name().to_string();
        generate(self.shell, &mut cmd, bin_name, &mut io::stdout());

        Ok(())
    }
}
