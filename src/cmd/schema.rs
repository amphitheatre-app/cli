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

use clap::{Arg, ArgMatches, Command};

pub fn build() -> Command<'static> {
    Command::new("schema")
        .about("List JSON schemas used to validate .amp.yaml configuration")
        .subcommand(Command::new("get")
            .about("Print a given .amp.yaml's json schema")
            .arg(Arg::new("options").takes_value(true))
            .after_help(super::AFTER_HELP_STRING)
        )
        .after_help(super::AFTER_HELP_STRING)
}

pub fn execute(args: &ArgMatches) {
    todo!()
}