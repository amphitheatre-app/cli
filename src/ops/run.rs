// Copyrgiht 2023 The Amphitheatre Authors.
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

use amp_client::client::Client;
use amp_client::playbooks::PlaybookPayload;
use amp_common::filesystem::Finder;
use amp_common::schema::{Manifest, Source};

use crate::errors::Result;

pub fn run() -> Result<()> {
    // Handle signal.
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Validate

    // Create playbook from this Character
    let path = Finder::new().find().expect("Config file .amp.toml not found");
    let contents = std::fs::read_to_string(path)?;
    let manifest: Manifest = toml::from_str(&contents)?;

    let payload = PlaybookPayload {
        title: "Untitled".to_string(),
        description: "".to_string(),
        preface: Source::new(manifest.character.repository),
    };

    let client = Client::new("http://localhost:8170/v1", None);
    let playbook = client.playbooks().create(payload)?;

    // // Sync the source to remote Dev Container
    // if let Err(e) = sync(".".to_string(), src(&playbook.lead())) {
    //     eprintln!("Error: Could not sync the sources ({})", e);
    //     std::process::exit(1);
    // }

    // Run
    // if let Err(e) = client.playbooks().start(&playbook.id) {
    //     eprintln!("Error: Could not start playbook {} ({})", &playbook.title, e);
    //     std::process::exit(1);
    // }

    println!("The playbook was created and deployed successfully!");
    println!("Visit: http://{}.amphitheatre.app", &playbook.id);

    // Read event stream looply.
    // loop {
    //     let event = client.playbooks().events(&playbook.id);
    //     println!("Received event: {}", event);

    //     thread::sleep(Duration::from_secs(2));
    // }

    Ok(())
}
