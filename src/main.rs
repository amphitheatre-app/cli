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

mod cmd;
mod context;
mod errors;
mod ops;
mod platform;
mod utils;

use std::sync::Arc;

use clap::Parser;
use context::Context;
use errors::Result;
use tracing::error;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::cmd::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
    tracing_subscriber::fmt().without_time().with_target(false).with_env_filter(filter).init();

    let ctx = Arc::new(Context::init().await?);
    match Cli::parse().exec(ctx).await {
        Ok(_) => {}
        Err(err) => {
            error!("{:#}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
