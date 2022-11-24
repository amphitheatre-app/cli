// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::character::Character;
use crate::manifest::VirtualManifest;
use errors::{Ok, Result};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Workspace {
    // This path is a path to where the current amp subcommand was invoked from.
    // that is the `--manifest-path` argument to amp, and points to the "main
    // character" that we're going to worry about.
    current_manifest: PathBuf,

    // A list of characters found in this workspace. Always includes at least
    // the character mentiioned by `current_manifest`.
    characters: Characters,

    // If this workspace inlucdes more than one character, this points to the
    // root of the workspace. This is `None` in the case that `[workspace]` is
    // missing, `character.workspace` is missing, and no `.amp.toml` above
    // `current_manifest` was found on the filesystem with `[workspace]`.
    root_manifest: Option<PathBuf>,

    // List of members in this workspace with a listing of all their manifest
    // paths. The character themselves can be looked up through the `characters`
    // set above.
    members: Vec<PathBuf>,
}

// Separate structure for tracking loaded characters (to avoid loading anything
// twice), and this is separate to help appease the borrow checker.
#[derive(Debug)]
struct Characters {
    characters: HashMap<PathBuf, MaybeCharacter>,
}

#[derive(Debug)]
pub enum MaybeCharacter {
    Character(Character),
    Virtual(VirtualManifest),
}

impl Workspace {
    /// Creates a new workspace given the target manifest pointed to by
    /// `manifest_path`.
    ///
    /// This function will construct the entire workspace by determining the
    /// root and all member packages. It will then validate the workspace
    /// before returning it, so `Ok` is only returned for valid workspaces.
    pub fn new() -> Result<Workspace> {
        let ws = Workspace::default();
        Ok(ws)
    }

    fn default() -> Workspace {
        Workspace {
            current_manifest: PathBuf::new(),
            characters: Characters {
                characters: HashMap::new(),
            },
            root_manifest: None,
            members: Vec::new(),
        }
    }

    /// Returns the current character of this workspace.
    ///
    /// Note that this can return an error if it the current manifest is
    /// actually a "virtual .amp.toml", in which case an error is returned
    /// indicating that something else should be passed.
    pub fn current(&self) -> Result<&Character> {
        let character = self.current_opt().ok_or_else(|| {
            errors::format_err!(
                "manifest path `{}` is a virtual manifest, but this \
                 command requires running against an actual character in \
                 this workspace",
                self.current_manifest.display()
            )
        })?;
        Ok(character)
    }

    pub fn current_opt(&self) -> Option<&Character> {
        match *self.characters.get(&self.current_manifest) {
            MaybeCharacter::Character(ref p) => Some(p),
            MaybeCharacter::Virtual(..) => None,
        }
    }

    /// Returns the root path of this workspace.
    ///
    /// That is, this returns the path of the directory containing the
    /// `.amp.toml` which is the root of this workspace.
    pub fn root(&self) -> &Path {
        self.root_manifest().parent().unwrap()
    }

    /// Returns the path of the `.amp.toml` which is the root of this
    /// workspace.
    pub fn root_manifest(&self) -> &Path {
        self.root_manifest
            .as_ref()
            .unwrap_or(&self.current_manifest)
    }

    /// Returns an iterator over all characters in this workspace
    pub fn members(&self) -> impl Iterator<Item = &Character> {
        let characters = &self.characters;
        self.members
            .iter()
            .filter_map(move |path| match characters.get(path) {
                &MaybeCharacter::Character(ref p) => Some(p),
                _ => None,
            })
    }
}

impl Characters {
    fn get(&self, manifest_path: &Path) -> &MaybeCharacter {
        self.maybe_get(manifest_path).unwrap()
    }

    fn maybe_get(&self, manifest_path: &Path) -> Option<&MaybeCharacter> {
        self.characters.get(manifest_path.parent().unwrap())
    }
}
