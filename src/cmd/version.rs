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

use clap::{ArgMatches, Command};
use errors::Result;

pub fn build() -> Command<'static> {
    Command::new("version")
        .about("Print the version information")
        .after_help(super::AFTER_HELP_STRING)
}

pub fn execute(_args: &ArgMatches) -> Result<()> {
    println!("amp 0.1.0");
    Ok(())
}
