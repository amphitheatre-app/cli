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
    Command::new("config")
        .about("Interact with the global Amphitheatre config file (defaults to $HOME/.amp/config)")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("List all values set in the global Amphitheatre config")
                .args(&[
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .takes_value(false)
                        .help("Show values for all kubecontexts"),
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .default_value("$HOME/.amp/config")
                        .help("Path to Amphitheatre config"),
                    Arg::new("kube-context")
                        .long("kube-context")
                        .takes_value(true)
                        .help("Kubectl context to set values against"),
                ])
                .after_help(super::AFTER_HELP_STRING),
        )
        .subcommand(
            Command::new("set")
                .about("Set a value in the global Amphitheatre config")
                .args(&[
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .default_value("$HOME/.amp/config")
                        .help("Path to Amphitheatre config"),
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .takes_value(false)
                        .help("Set value for global config"),
                    Arg::new("kube-context")
                        .long("kube-context")
                        .takes_value(true)
                        .help("Kubectl context to set values against"),
                ])
                .after_help(super::AFTER_HELP_STRING),
        )
        .subcommand(
            Command::new("unset")
                .about("Unset a value in the global Amphitheatre config")
                .args(&[
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .default_value("$HOME/.amp/config")
                        .help("Path to Amphitheatre config"),
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .takes_value(false)
                        .help("Set value for global config"),
                    Arg::new("kube-context")
                        .long("kube-context")
                        .takes_value(true)
                        .help("Kubectl context to set values against"),
                ])
                .after_help(super::AFTER_HELP_STRING),
        )
        .after_help(super::AFTER_HELP_STRING)
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    todo!()
}
