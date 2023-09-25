// Copyright 2023 The Amphitheatre Authors.
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

use std::path::Path;

use amp_common::schema::Character;

use crate::errors::{Errors, Result};

/// Read manifest file, validate and return content
pub fn read_manifest<P: AsRef<Path>>(path: P) -> Result<Character> {
    let content = std::fs::read_to_string(path).map_err(|e| Errors::FailedLoadManifest(e.to_string()))?;
    toml::from_str::<Character>(&content).map_err(Errors::InvalidManifest)
}
