// Copyright 2023 The Amphitheatre Authors.
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

use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::schema::EitherCharacter::Manifest;

use crate::context::Context;
use crate::errors::Result;

pub async fn run(ctx: Arc<Context>) -> Result<()> {
    // Create playbook from this Character
    let path = Finder::new().find().expect("Config file .amp.toml not found");
    let content = std::fs::read_to_string(path)?;

    let payload = PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: Manifest(content),
        live: false,
    };
    display!("{:#?}", payload);

    let context = ctx.context().await?;
    let client = Client::new(&format!("{}/v1", &context.server), context.token);

    let playbook = client.playbooks().create(payload)?;
    println!("The playbook was created and deployed successfully!");
    display!("{:#?}", playbook);

    Ok(())
}