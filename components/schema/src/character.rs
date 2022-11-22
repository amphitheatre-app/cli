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

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Character {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub readme: String,
    pub homepage: String,
    pub repository: String,
    pub license: String,
    pub license_file: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
    pub publish: Vec<String>,
}
