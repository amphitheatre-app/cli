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

use clap::{App, ArgMatches, Command};
use errors::Result;

mod apply;
mod build;
mod clean;
mod completion;
mod config;
mod debug;
mod deploy;
mod dev;
mod diagnose;
mod fix;
mod init;
mod options;
mod render;
mod run;
mod schema;
mod survey;
mod test;
mod version;

type CmdName = &'static str;
type FnBuiler = fn() -> App<'static>;
type FnExcuter = for<'r> fn(&'r ArgMatches) -> Result<()>;

lazy_static::lazy_static! {
    static ref COMMANDS: Vec<(CmdName, FnBuiler, FnExcuter)> = vec![
        ("apply", apply::build, apply::execute),
        ("build", build::build, build::execute),
        ("clean", clean::build, clean::execute),
        ("completion", completion::build, completion::execute),
        ("config", config::build, config::execute),
        ("debug", debug::build, debug::execute),
        ("deploy", deploy::build, deploy::execute),
        ("dev", dev::build, dev::execute),
        ("diagnose", diagnose::build, diagnose::execute),
        ("fix", fix::build, fix::execute),
        ("init", init::build, init::execute),
        ("options", options::build, options::execute),
        ("render", render::build, render::execute),
        ("run", run::build, run::execute),
        ("schema", schema::build, schema::execute),
        ("survey", survey::build, survey::execute),
        ("test", test::build, test::execute),
        ("version", version::build, version::execute),
    ];
}

const AFTER_HELP_STRING: &'static str =
    "Use \"amp options\" for a list of global \
    command-line options (applies to all commands).";

pub fn build() -> Command<'static> {
    Command::new("amp")
        .about("Amphitheatre's offcial command line tool")
        .arg_required_else_help(true)
        .subcommands(
            COMMANDS
                .iter()
                .map(move |(_, build, _)| build())
                .collect::<Vec<App>>()
        )
        .after_help(AFTER_HELP_STRING)
}

pub fn execute() {
    let matches = build().get_matches();

    if let Some((name, args)) = matches.subcommand() {
        if let Some((_, _, execute)) =
            COMMANDS.iter().find(|&&(cmd, _, _)| cmd == name) {
            if let Err(e) = execute(args) {
                println!("Failed to execute the {} command, error is {}",
                name, e.to_string());
            }
        }
    }
}
