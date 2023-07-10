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

use clap::Args;

use crate::context::Context;
use crate::errors::Result;

/// Select one of your existing contexts or to create a new one
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {
    name: Option<String>,
}

impl Cli {
    pub async fn exec(&self, _ctx: Arc<Context>) -> Result<()> {
        Ok(())
    }
}