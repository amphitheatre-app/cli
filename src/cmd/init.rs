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

use std::path::Path;
use clap::{Arg, ArgAction, ArgMatches, Command};
use errors::Result;
use std::fs;
use schema::Manifest;

const FILE_NAME: &str = ".amp.toml";

pub fn build() -> Command<'static> {
    Command::new("init")
        .about("Generate configuration for deploying an application")
        .args(&[
            Arg::new("name")
                .long("name")
                .takes_value(true)
                .help("Set the character name. Defaults to the directory name."),
            Arg::new("force")
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force the generation of the Amphitheatre character"),
        ])
        .after_help(super::AFTER_HELP_STRING)
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let path = std::env::current_dir()?;

    let name = args
        .get_one::<String>("name")
        .map(String::as_str)
        .or_else(||name(&path).ok())
        .unwrap();
    let force = args.get_flag("force");

    if !force && path.join(FILE_NAME).exists() {
        println!("`amp init` cannot be run on existing Amphitheatre character");
        std::process::exit(1);
    }

    if let Err(e) = create(name) {
        println!("Failed to create the character: {}", e.to_string());
        std::process::exit(1);
    }

    println!("Created the character: {}, see more manfifest at .amp.toml", name);

    Ok(())
}

fn name<'a>(path: &'a Path) -> Result<&'a str> {
    let file_name = path.file_name().ok_or_else(|| {
        errors::format_err!(
            "cannot auto-detect character name from path {:?} ; use --name to override",
            path.as_os_str()
        )
    })?;

    file_name.to_str().ok_or_else(|| {
        errors::format_err!(
            "cannot create character with a non-unicode name: {:?}",
            file_name
        )
    })
}

fn create<'a>(name: &'a str) -> Result<()> {
    // Init and fill the Manifest fields.
    let mut manifest = Manifest::default();
    manifest.character.name = String::from(name);

    // Convert the Manifest to a TOML String.
    let serialized = toml::to_string(&manifest).expect("Could not encode TOML value");
    fs::write(FILE_NAME, serialized).expect("Could not write to file!");

    Ok(())
}