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

use std::fmt::Display;
use std::sync::Arc;

use amp_common::config::{Cluster, Configuration};
use clap::Args;
use inquire::error::InquireResult;
use inquire::{Password, Select, Text};

use crate::context::Context;
use crate::errors::{Errors, Result};

const CREATE_KEY: &str = "$$CREATE$$";

/// Select one of your existing contexts or to create a new one
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    name: Option<String>,
}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        if let Some(name) = &self.name {
            return use_context(ctx, name).await;
        }

        // display the available contexts for selection
        let answer = select_context(ctx.clone()).await?;

        // if the user selects "create new context", create a new context
        if answer.0 == CREATE_KEY {
            return create_context(ctx.clone()).await;
        }

        // if the user selects a context, set it as the current context
        // and set it as the current context
        use_context(ctx.clone(), answer.0.as_str()).await?;

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
async fn select_context(ctx: Arc<Context>) -> Result<OptionItem> {
    let configuration = ctx.configuration.read().await;
    let context = configuration.context.as_ref().ok_or(Errors::NotFoundContexts)?;

    // create a options with the available contexts
    let mut options: Vec<OptionItem> =
        context.iter().map(|(name, ctx)| OptionItem(String::from(name), ctx.title.clone())).collect();
    options.push(OptionItem(CREATE_KEY.into(), "Create new context".into()));

    // run the select prompt
    let answer = Select::new("Select the context:", options).prompt().map_err(Errors::InquireError)?;

    Ok(answer)
}

#[derive(PartialEq)]
struct OptionItem(String, String);

impl Display for OptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

/// Create a new context
async fn create_context(ctx: Arc<Context>) -> Result<()> {
    let mut configuration = ctx.configuration.write().await;
    let context = configuration.context.as_mut().ok_or(Errors::NotFoundContexts)?;

    let (name, cluster) = inquire().map_err(Errors::InquireError)?;
    context.add(&name, cluster).map_err(Errors::FailedAddContext)?;
    configuration
        .save(Configuration::path().map_err(Errors::InvalidConfigPath)?)
        .map_err(Errors::FailedSaveConfiguration)?;

    Ok(())
}

fn inquire() -> InquireResult<(String, Cluster)> {
    let mut cluster = Cluster::default();

    let name = Text::new("What is the name of the context?").prompt()?;
    cluster.title = Text::new("What is the title of the context?").prompt()?;
    cluster.server = Text::new("What is the server address of the cluster?").prompt()?;
    cluster.token = Some(Password::new("What is the token of the cluster?").prompt()?);

    Ok((name, cluster))
}
