// Copyright 2022 The Amphitheatre Authors.
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

use amp_common::schema::Manifest;
use client::playbooks::PlaybookPayload;

use crate::common::setup_mock_for;
mod common;

#[test]
fn list_playbooks_test() {
    let setup = setup_mock_for("/playbooks", "playbooks/list-playbooks-success", "GET");
    let client = setup.0;

    let playbooks = client.playbooks().list(None).unwrap().data.unwrap();

    assert_eq!(2, playbooks.len());

    let playbook = playbooks.first().unwrap();

    assert_eq!(1, playbook.id);
    assert_eq!("Default", playbook.title);
    assert_eq!("First", playbook.description);
    assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[test]
fn create_playbook_test() {
    let setup = setup_mock_for("/playbooks", "playbooks/create-playbook-created", "POST");
    let client = setup.0;

    let payload = PlaybookPayload {
        title: String::from("Default"),
        description: String::from("First"),
        lead: Manifest::default(),
    };

    let playbook = client.playbooks().create(payload).unwrap().data.unwrap();

    assert_eq!(1, playbook.id);
    assert_eq!("Default", playbook.title);
    assert_eq!("First", playbook.description);
}

#[test]
fn get_playbook_test() {
    let setup = setup_mock_for("/playbooks/1", "playbooks/get-playbook-success", "GET");
    let client = setup.0;
    let playbook_id = 1;

    let playbook = client.playbooks().get(playbook_id).unwrap().data.unwrap();

    assert_eq!(1, playbook.id);
    assert_eq!("Default", playbook.title);
    assert_eq!("First", playbook.description);
    assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[test]
fn update_playbook_test() {
    let setup = setup_mock_for("/playbooks/1", "playbooks/update-playbook-success", "PATCH");
    let client = setup.0;
    let playbook_id = 1;

    let payload = PlaybookPayload {
        title: String::from("Default"),
        description: String::from("First"),
        lead: Manifest::default(),
    };

    let playbook = client
        .playbooks()
        .update(playbook_id, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(1, playbook.id);
    assert_eq!("Default", playbook.title);
    assert_eq!("First", playbook.description);
}

#[test]
fn delete_playbook_test() {
    let setup = setup_mock_for("/playbooks/1", "playbooks/delete-playbook-success", "DELETE");
    let client = setup.0;
    let playbook_id = 1;

    let response = client.playbooks().delete(playbook_id);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[test]
fn get_playbook_events() {
    let setup = setup_mock_for(
        "/playbooks/1/events",
        "playbooks/get-playbook-events-success",
        "GET",
    );
    let client = setup.0;
    let playbook_id = 1;

    let response = client.playbooks().events(playbook_id);

    assert_eq!(String::from("event stream (JSON)"), response);
}

#[test]
fn start_playbook_test() {
    let setup = setup_mock_for(
        "/playbooks/1/actions/start",
        "playbooks/start-playbook-success",
        "POST",
    );
    let client = setup.0;
    let playbook_id = 1;

    let response = client.playbooks().start(playbook_id);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}

#[test]
fn stop_playbook_test() {
    let setup = setup_mock_for(
        "/playbooks/1/actions/stop",
        "playbooks/stop-playbook-success",
        "POST",
    );
    let client = setup.0;
    let playbook_id = 1;

    let response = client.playbooks().stop(playbook_id);

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap().status);
}
