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
use errors::Result;

pub fn build() -> Command<'static> {
    Command::new("options")
        .about("Output a list of global command-line options (applies to all commands)")
        .args(&[
            Arg::new("color")
                .long("color")
                .default_value("34")
                .help("Specify the default output color in ANSI escape codes"),
            Arg::new("interactive")
                .long("interactive")
                .takes_value(false)
                .help("Allow user prompts for more information"),
            Arg::new("timestamps")
                .long("timestamps")
                .takes_value(false)
                .help("Print timestamps in logs"),
            Arg::new("update-check")
                .long("update-check")
                .takes_value(false)
                .help("Check for a more recent version of Amphitheatre"),
            Arg::new("verbosity")
                .short('v')
                .long("verbosity")
                .default_value("warning")
                .help("Log level: one of [panic fatal error warning info debug trace]"),
        ])
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    todo!()
}
