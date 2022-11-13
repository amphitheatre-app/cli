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

use clap::Command;

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

pub fn build() -> Command<'static> {
    Command::new("amp")
        .about("Amphitheatre's offcial command line tool")
        .arg_required_else_help(true)

        .subcommand(apply::build())
        .subcommand(build::build())
        .subcommand(clean::build())
        .subcommand(completion::build())
        .subcommand(config::build())
        .subcommand(debug::build())
        .subcommand(deploy::build())
        .subcommand(dev::build())
        .subcommand(diagnose::build())
        .subcommand(fix::build())
        .subcommand(init::build())
        .subcommand(options::build())
        .subcommand(render::build())
        .subcommand(run::build())
        .subcommand(schema::build())
        .subcommand(survey::build())
        .subcommand(test::build())
        .subcommand(version::build())
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}

pub fn execute() {
    let matches = build().get_matches();

    match matches.subcommand() {
        Some(("apply", args)) => apply::execute(args),
        Some(("build", args)) => build::execute(args),
        Some(("clean", args)) => clean::execute(args),
        Some(("completion", args)) => completion::execute(args),
        Some(("config", args)) => config::execute(args),
        Some(("debug", args)) => debug::execute(args),
        Some(("deploy", args)) => deploy::execute(args),
        Some(("dev", args)) => dev::execute(args),
        Some(("diagnose", args)) => diagnose::execute(args),
        Some(("fix", args)) => fix::execute(args),
        Some(("init", args)) => init::execute(args),
        Some(("options", args)) => options::execute(args),
        Some(("render", args)) => render::execute(args),
        Some(("run", args)) => run::execute(args),
        Some(("schema", args)) => schema::execute(args),
        Some(("survey", args)) => survey::execute(args),
        Some(("test", args)) => test::execute(args),
        Some(("version", args)) => version::execute(args),
        _ => unreachable!(),
    }
}