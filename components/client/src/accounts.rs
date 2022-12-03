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

use serde::{Deserialize, Serialize};

use super::client::{Client, Endpoint, Response};
use super::errors::ClientError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    /// The account ID
    pub id: u64,
    /// The account email
    pub email: String,
    /// The account name
    pub name: String,
    /// When the account was created in Amphitheatre
    pub created_at: String,
    /// When the account was updated in Amphitheatre
    pub updated_at: String,
}

struct AccountEndpoint;

impl Endpoint for AccountEndpoint {
    type Output = Account;
}

/// The Accounts Service handles the account endpoint of the Amphitheatre API.
///
/// See [API Documentation: Account](https://docs.amphitheatre.app/api/account)
pub struct Accounts<'a> {
    pub client: &'a Client,
}

impl Accounts<'_> {
    /// Retrieves the details about the current authenticated entity used to acces the API.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use client::client::Client;
    ///
    /// let client = Client::new(
    ///     String::from("https://cloud.amphitheatre.app"),
    ///     String::from("AUTH_TOKEN"),
    /// );
    /// let response = client.accounts().me().unwrap();
    /// let account = response.data.unwrap();
    /// ```
    pub fn me(&self) -> Result<Response<Account>, ClientError> {
        self.client.get::<AccountEndpoint>("/me", None)
    }
}
