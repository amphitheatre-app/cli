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

use assert_matches::assert_matches;
use client::errors::ClientError;
use serde_json::json;

use crate::common::setup_mock_for;

mod common;

#[test]
fn validation_error() {
    let setup = setup_mock_for("/me", "errors/validation-error", "GET");
    let client = setup.0;

    let response = client.accounts().me();
    let error = response.unwrap_err();

    assert_eq!("Validation failed", error.to_string());
    assert_matches!(error, ClientError::BadRequest{ message, attribute_errors } => {
      assert_eq!("Validation failed", message);
      assert_eq!(json!({"address1":["can't be blank"],"city":["can't be blank"],"country":["can't be blank"],"email":["can't be blank","is an invalid email address"],"first_name":["can't be blank"],"last_name":["can't be blank"],"phone":["can't be blank","is probably not a phone number"],"postal_code":["can't be blank"],"state_province":["can't be blank"]}), attribute_errors.unwrap());
    })
}

#[test]
fn not_found() {
    let setup = setup_mock_for("/me", "errors/notfound-certificate", "GET");
    let client = setup.0;

    let response = client.accounts().me();
    let error = response.unwrap_err();

    assert_eq!("Certificate `0` not found", error.to_string());
}

#[test]
fn method_not_allowed() {
    let setup = setup_mock_for("/me", "errors/method-not-allowed", "GET");
    let client = setup.0;

    let response = client.accounts().me();
    let error = response.unwrap_err();

    assert_eq!("Method not Allowed", error.to_string());
}

#[test]
fn bad_gateway() {
    let setup = setup_mock_for("/me", "errors/bad-gateway", "GET");
    let client = setup.0;

    let response = client.accounts().me();
    let error = response.unwrap_err();

    assert_eq!("Bad Gateway", error.to_string());
}
#[test]
fn transport() {
    let setup = setup_mock_for("/other", "errors/bad-gateway", "GET");
    let client = setup.0;

    let response = client.accounts().me();
    let error = response.unwrap_err();

    assert_eq!("Transport Error - 501(Mock Not Found)", error.to_string());
}
