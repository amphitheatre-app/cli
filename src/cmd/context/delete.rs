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

use amp_common::config::Configuration;
use clap::Args;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Delete a context
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    name: String,
}

impl Cli {
    // delete the context and save the contexts
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let mut configuration = ctx.configuration.write().await;

        if let Some(context) = configuration.context.as_mut() {
            context.delete(&self.name).map_err(Errors::FailedDeleteContext)?
        } else {
            return Err(Errors::NotFoundContext(self.name.clone()));
        }

        configuration
            .save(Configuration::path().map_err(Errors::InvalidConfigPath)?)
            .map_err(Errors::FailedSaveConfiguration)?;

        Ok(())
    }
}
