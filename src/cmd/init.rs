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
use schema::Manifest;
use std::fs;

const FILE_NAME: &str = ".amp.toml";

/// Create a new Amphitheatre character in an existing directory
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// Set the character name. Defaults to the directory name.
    #[arg(long)]
    name: Option<String>,
    /// Force the generation of the Amphitheatre character
    #[arg(long, action= clap::ArgAction::SetTrue)]
    force: bool,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let path = std::env::current_dir().unwrap();

        let name = self
            .name
            .as_deref()
            .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap());

        if !self.force && path.join(FILE_NAME).exists() {
            println!("`amp init` cannot be run on existing Amphitheatre character");
            std::process::exit(1);
        }

        if let Err(e) = create(name) {
            println!("Failed to create the character: {}", e);
            std::process::exit(1);
        }

        println!(
            "Created the character: {}. See more definitions at `.amp.toml`",
            name
        );

        Ok(())
    }
}

fn create(name: &str) -> Result<()> {
    // Init and fill the Manifest fields.
    let mut manifest = Manifest::default();
    manifest.character.name = String::from(name);

    // Convert the Manifest to a TOML String.
    let serialized = toml::to_string(&manifest).expect("Could not encode TOML value");
    fs::write(FILE_NAME, serialized).expect("Could not write to file!");

    Ok(())
}
