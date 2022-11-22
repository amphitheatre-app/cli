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

use crate::common::setup_mock_for;

mod common;

#[test]
fn me_success_with_account() {
    let setup = setup_mock_for("/me", "accounts/get-me-success", "GET");
    let client = setup.0;
    let response = client.accounts().me().unwrap();

    let account = response.data.unwrap();

    assert_eq!(1, account.id);
    assert_eq!("example-account@example.com", account.email);
    assert_eq!("example-account", account.name);
}
