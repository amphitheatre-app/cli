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

//! Inspired by dotenv::find::Finder

use std::path::{Path, PathBuf};
use std::{env, fs, io};

use crate::errors::*;

pub struct Finder<'a> {
    filename: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new() -> Self {
        Finder {
            filename: Path::new(".amp.toml"),
        }
    }

    pub fn filename(mut self, filename: &'a Path) -> Self {
        self.filename = filename;
        self
    }

    pub fn find(self) -> Result<PathBuf> {
        find(&env::current_dir().map_err(Error::Io)?, self.filename)
    }
}

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find(directory: &Path, filename: &Path) -> Result<PathBuf> {
    let candidate = directory.join(filename);

    match fs::metadata(&candidate) {
        Ok(metadata) => {
            if metadata.is_file() {
                return Ok(candidate);
            }
        }
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(Error::Io(error));
            }
        }
    }

    if let Some(parent) = directory.parent() {
        find(parent, filename)
    } else {
        Err(Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "path not found",
        )))
    }
}
