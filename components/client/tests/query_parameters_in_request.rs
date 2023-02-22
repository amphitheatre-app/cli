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

use std::collections::HashMap;

use crate::common::setup_mock_for;
mod common;
use client::client::{Endpoint, Filters, Paginate, RequestOptions, Sort};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Id {
    #[allow(dead_code)] // Unread but required for test fixture
    pub id: u64,
}

struct IdsEndpoint;

impl Endpoint for IdsEndpoint {
    type Output = Vec<Id>;
}

#[test]
fn can_paginate() {
    let setup = setup_mock_for("/pagination_test?page=2&per_page=2", "requests/pages-2of3", "GET");
    let client = setup.0;
    let options = RequestOptions {
        filters: None,
        sort: None,
        paginate: Some(Paginate { per_page: 2, page: 2 }),
    };

    client
        .get::<IdsEndpoint>("/pagination_test", Some(options))
        .unwrap();
}

#[test]
fn can_filter() {
    let setup = setup_mock_for("/filter_test?name_like=example", "requests/pages-2of3", "GET");
    let client = setup.0;
    let mut filters = HashMap::new();
    filters.insert("name_like".to_string(), "example".to_string());
    let options = RequestOptions {
        filters: Some(Filters { filters }),
        sort: None,
        paginate: None,
    };

    client.get::<IdsEndpoint>("/filter_test", Some(options)).unwrap();
}

#[test]
fn can_sort() {
    let setup = setup_mock_for("/sort_test?sort=expiration%3Aasc", "requests/pages-2of3", "GET");
    let client = setup.0;
    let options = RequestOptions {
        filters: None,
        sort: Some(Sort {
            sort_by: "expiration:asc".to_string(),
        }),
        paginate: None,
    };

    client.get::<IdsEndpoint>("/sort_test", Some(options)).unwrap();
}
