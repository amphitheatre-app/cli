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

use amp_common::config::Configuration;
use clap::Args;
use inquire::Select;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Delete a context
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// The name of the context to delete
    name: Option<String>,
}

impl Cli {
    // delete the context and save the contexts
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        if let Some(name) = &self.name {
            return delete(&ctx, name).await;
        }

        // display the available contexts for selection
        let answer = select_context(&ctx).await?;
        delete(&ctx, answer.0.as_str()).await?;

        Ok(())
    }
}

/// Select the context with the given name
async fn select_context(ctx: &Arc<Context>) -> Result<OptionItem> {
    let configuration = ctx.configuration.read().await;
    let context = configuration.context.as_ref().ok_or(Errors::NotFoundContexts)?;

    // create a options with the available contexts
    let options: Vec<OptionItem> =
        context.iter().map(|(name, ctx)| OptionItem(String::from(name), ctx.title.clone())).collect();

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

async fn delete(ctx: &Arc<Context>, name: &str) -> Result<()> {
    let mut configuration = ctx.configuration.write().await;
    let context = configuration.context.as_mut().ok_or(Errors::NotFoundContexts)?;

    context.delete(name).map_err(Errors::FailedDeleteContext)?;
    configuration
        .save(Configuration::path().map_err(Errors::InvalidConfigPath)?)
        .map_err(Errors::FailedSaveConfiguration)?;

    Ok(())
}
