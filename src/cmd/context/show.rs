// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use clap::Args;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Print the current context
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let configuration = ctx.configuration.read().await;
        let context = configuration.context.as_ref().ok_or(Errors::NotFoundContexts)?;

        if let Some((_, current)) = context.current() {
            println!("{current:#?}");
        } else {
            println!("No current context");
        }

        Ok(())
    }
}
