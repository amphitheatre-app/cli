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

use std::fs;
use std::sync::Arc;

use crate::context::Context;
use crate::errors::{Errors, Result};
use amp_common::schema::Character;
use clap::Args;
use colored::Colorize;
use tracing::error;

const FILE_NAME: &str = ".amp.toml";

/// Create a new Amphitheatre character in an existing directory
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true", env = "AMP_ASSUME_YES")]
    assume_yes: bool,
    /// File to write generated manifests to
    #[arg(short, long, default_value = ".amp.toml", env = "AMP_FILENAME")]
    filename: Option<String>,
    /// Force the generation of the Amphitheatre character
    #[arg(long, action = clap::ArgAction::SetTrue, env = "AMP_FORCE")]
    force: bool,
    /// Set the character name. Defaults to the directory name.
    #[arg(long, env = "AMP_NAME")]
    name: Option<String>,
}

impl Cli {
    pub async fn exec(&self, _ctx: Arc<Context>) -> Result<()> {
        let path = std::env::current_dir().unwrap();

        let name = self.name.as_deref().unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap());

        if !self.force && path.join(FILE_NAME).exists() {
            error!("`amp init` cannot be run on existing Amphitheatre character");
            std::process::exit(1);
        }

        if let Err(e) = create(name) {
            error!("Failed to create the character: {}", e.to_string());
            std::process::exit(1);
        }

        println!("Configuration .amp.toml was created successfully");
        println!("{}", "You can now run [amp run] to build and deploy your character".green());
        println!("{}", "or [amp dev] to enter development mode, with hot reloading".green());

        Ok(())
    }
}

fn create(name: &str) -> Result<()> {
    // Init and fill the Manifest fields.
    let manifest = Character::new(name);

    // Convert the Manifest to a TOML String.
    let serialized = toml::to_string(&manifest).map_err(Errors::TomlSerializeError)?;
    println!("{serialized}");
    fs::write(FILE_NAME, serialized).map_err(Errors::FailedSaveManifest)?;

    Ok(())
}
