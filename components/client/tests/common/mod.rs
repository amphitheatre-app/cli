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

// Heavily inspired by https://github.com/dnsimple/dnsimple-rust

use std::fs;

use client::client::Client;
use mockito::{mock, Mock};

/// Creates a mockserver and a client (changing the url of the client
/// to that of the mockserver to capture the requests).
///
/// It builds a response struct for the mock server using the fixture.
///
/// # Arguments
///
/// `fixture`: the path to the fixture inside the `api` directory
/// `path`: the path in the server (i.e. `/me`)
/// `method`: the HTTP method we are going to use (GET, POST, DELETE, ...)
pub fn setup_mock_for(path: &str, fixture: &str, method: &str) -> (Client, Mock) {
    let path = format!("/v1{}", path);
    let fixture = format!("./tests/fixtures/v1/api/{}.http", fixture);

    let content = fs::read_to_string(fixture.as_str()).expect("Something went wrong: Couldn't read the file");

    let lines = content.lines();
    let status = &content[9..12];
    let body = lines.last();

    let mock = mock(method, path.as_str())
        .with_header("x-ratelimit-limit", "2")
        .with_header("x-ratelimit-remaining", "2")
        .with_header("x-ratelimit-after", "never")
        .with_status(status.parse().unwrap())
        .with_body(body.unwrap())
        .create();

    let client = Client::new(mockito::server_url(), String::from("some-token"));

    (client, mock)
}
