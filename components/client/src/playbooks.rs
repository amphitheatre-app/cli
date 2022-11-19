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

use schema::Playbook;
use crate::client::{Client, RequestOptions, Response, Endpoint, EmptyResponse};
use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybookPayload {
    /// The title of the playbook
    pub title: String,
    /// The description of the playbook
    pub description: String,
    /// The lead character url of the playbook
    pub lead: String,
}

struct PlaybookEndpoint;

impl Endpoint for PlaybookEndpoint {
    type Output = Playbook;
}

struct PlaybooksEndpoint;

impl Endpoint for PlaybooksEndpoint {
    type Output = Vec<Playbook>;
}

/// The Playbooks Service handles the playbooks endpoint of the Amphitheatre API.
///
/// See [API Documentation: playbook](https://docs.amphitheatre.app/api/playbook)
pub struct Playbooks<'a> {
    pub client: &'a Client,
}

impl Playbooks<'_> {
    /// Lists the playbooks in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `options`: The `RequestOptions`
    ///             - Sort: `id`, `label`, `email`
    pub fn list(
        &self,
        account_id: u64,
        options: Option<RequestOptions>,
    ) -> Result<Response<Vec<Playbook>>, Error> {
        let path = format!("/{}/playbooks", account_id);
        self.client.get::<PlaybooksEndpoint>(&*path, options)
    }

    /// Create a playbook in the account.
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `payload`: the `PlaybookPayload` with the information needed to create
    /// the playbook
    pub fn create(
        &self,
        account_id: u64,
        payload: PlaybookPayload,
    ) -> Result<Response<Playbook>, Error> {
        let path = format!("/{}/playbooks", account_id);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.post::<PlaybookEndpoint>(&*path, json),
            Err(_) => Err(Error::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieve a playbook
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `playbook_id`: The ID of the playbook we want to retrieve
    pub fn get(
        &self, account_id: u64, playbook_id: u64
    ) -> Result<Response<Playbook>, Error> {
        let path = format!("/{}/playbooks/{}", account_id, playbook_id);
        self.client.get::<PlaybookEndpoint>(&*path, None)
    }

    /// Update a playbook
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `playbook_id`: The playbook id
    /// `payload`: The `PlaybookPayload` with the information needed to update
    pub fn update(
        &self,
        account_id: u64,
        playbook_id: u64,
        payload: PlaybookPayload,
    ) -> Result<Response<Playbook>, Error> {
        let path = format!("/{}/playbooks/{}", account_id, playbook_id);

        match serde_json::to_value(payload) {
            Ok(json) => self.client.patch::<PlaybookEndpoint>(&*path, json),
            Err(_) => Err(Error::Deserialization(String::from(
                "Cannot deserialize json payload",
            )))
        }
    }

    /// Delete a playbook
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `playbook_id`: The playbook id
    pub fn delete(
        &self,
        account_id: u64,
        playbook_id: u64,
    ) -> Result<EmptyResponse, Error> {
        let path = format!("/{}/playbooks/{}", account_id, playbook_id);
        self.client.delete(&*path)
    }

    /// Start a playbook
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `playbook_id`: The playbook id
    pub fn start(
        &self,
        account_id: u64,
        playbook_id: u64,
    ) -> Result<EmptyResponse, Error> {
        let path = format!("/{}/playbooks/{}/actions/start", account_id, playbook_id);
        self.client.empty_post(&*path)
    }

    /// Stop a playbook
    ///
    /// # Arguments
    ///
    /// `account_id`: The account ID
    /// `playbook_id`: The playbook id
    pub fn stop(
        &self,
        account_id: u64,
        playbook_id: u64,
    ) -> Result<EmptyResponse, Error> {
        let path = format!("/{}/playbooks/{}/actions/stop", account_id, playbook_id);
        self.client.empty_post(&*path)
    }

    pub fn events(&self) -> String {
        String::from("event stream (JSON)")
    }
}