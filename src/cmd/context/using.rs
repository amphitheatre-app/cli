// Copyright 2023 The Amphitheatre Authors.
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

use std::sync::Arc;

use amp_common::config::{Cluster, Configuration};
use clap::Args;
use inquire::{Password, Select, Text};

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Select one of your existing contexts or to create a new one
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    name: Option<String>,
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        if let Some(name) = &self.name {
            use_context(ctx, name).await?;
        } else {
            select_context(ctx).await.map_err(Errors::FailedSelectContext)?;
        }

        Ok(())
    }
}

/// Set the current context with the given name
async fn use_context(ctx: Arc<Context>, name: &str) -> Result<()> {
    let mut configuration = ctx.configuration.write().await;
    let context = configuration.context.as_mut().ok_or(Errors::NotFoundContexts)?;

    context.select(name).map_err(Errors::FailedSelectContext)?;
    configuration
        .save(Configuration::path().map_err(Errors::InvalidConfigPath)?)
        .map_err(Errors::FailedSaveConfiguration)?;

    Ok(())
}

/// Select the context with the given name
async fn select_context(ctx: Arc<Context>) -> anyhow::Result<()> {
    let configuration = ctx.configuration.read().await;
    let context = configuration.context.as_ref().ok_or(Errors::NotFoundContexts)?;

    // create a options with the available contexts
    let mut options = context.iter().map(|ctx| ctx.title.as_str()).collect::<Vec<&str>>();
    options.push("Create new context");

    // run the select prompt
    let answer = Select::new("Select the context:", options).prompt();

    if let Err(err) = &answer {
        println!("Error: {:?}", err);
    }
    let answer = answer.unwrap();

    // if the user selects "create new context", create a new context
    if answer == "Create new context" {
        return Ok(create_context(ctx.clone()).await?);
    }
    // if the user selects a context, set it as the current context
    // and set it as the current context
    use_context(ctx.clone(), answer).await?;
    println!("Context set to {}", answer);

    Ok(())
}

/// Create a new context
async fn create_context(ctx: Arc<Context>) -> anyhow::Result<()> {
    let mut configuration = ctx.configuration.write().await;
    let context = configuration.context.as_mut().ok_or(Errors::NotFoundContexts)?;
    let mut cluster = Cluster::default();

    let name = Text::new("What is the name of the context?").prompt()?;
    println!("Result: {:?}", name);

    cluster.title = Text::new("What is the title of the context?").prompt()?;
    println!("Result: {:?}", cluster.title);

    cluster.server = Text::new("What is the server address of the cluster?").prompt()?;
    println!("Result: {:?}", cluster.server);

    cluster.token = Some(Password::new("What is the token of the cluster?").prompt()?);
    println!("Result: {:?}", cluster.token);

    context.add(&name, cluster)?;
    configuration
        .save(Configuration::path().map_err(Errors::InvalidConfigPath)?)
        .map_err(Errors::FailedSaveConfiguration)?;

    Ok(())
}
