// Copyrgiht 2023 The Amphitheatre Authors.
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

#[allow(unused_variables)]
#[allow(unused_macros)]
#[macro_use]
mod macros;
mod app;
mod cmd;
mod config;
mod context;
mod ops;
mod platform;

pub mod errors {
    pub use anyhow::*;
}

use clap::Parser;
use errors::Result;

use crate::cmd::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    app::set_global_verbosity(cli.verbose.log_level_filter());
    app::set_global_config(config::load()?);
    app::set_global_contexts(context::load()?);

    cli.exec()
}
