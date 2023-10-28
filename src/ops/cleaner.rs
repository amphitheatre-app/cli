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

use tokio::runtime::Runtime;
use tracing::{info, warn};

use crate::context::Context;
use crate::errors::{Errors, Result};

/// Setup handler for for handling Ctrl-C signals.
pub fn setup_signal_handler(ctx: Arc<Context>, cleanup: bool) {
    ctrlc::set_handler(move || {
        warn!("Received Ctrl-C, will exit now");

        if cleanup {
            // Try to delete playbook if it is available in the session.
            let context: Arc<Context> = ctx.clone();
            // need a tokio runtime to spawn a future, so we create one here.
            let rt = Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(async move {
                if let Err(err) = try_cleanup_playbook(&context).await {
                    warn!("Failed to cleanup playbook: {:?}", err);
                }
            });
        }

        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");
}

/// Try to delete playbook if it is available in the session.
pub async fn try_cleanup_playbook(ctx: &Arc<Context>) -> Result<()> {
    let playbook = ctx.session.playbook.read().await;

    // If there is no playbook in the session, just return.
    if playbook.is_none() {
        warn!("No playbook to cleanup");
        return Ok(());
    }

    // Delete playbook from the server.
    let pid = playbook.as_ref().unwrap().id.clone();
    let status = ctx.client.playbooks().delete(&pid).map_err(Errors::ClientError)?;
    if status != 204 {
        return Err(Errors::FailedDeletePlaybook(pid.to_string()));
    }

    // print success message
    info!("Deleted playbook {}", pid);

    Ok(())
}
