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

mod cmd;

fn main() {
    let matches = cmd::build().get_matches();

    match matches.subcommand() {
        Some(("apply",  _matches)) => {},
        Some(("build",  _matches)) => {},
        Some(("clean",  _matches)) => {},
        Some(("completion",  _matches)) => {},
        Some(("config",  _matches)) => {},
        Some(("debug",  _matches)) => {},
        Some(("deploy",  _matches)) => {},
        Some(("dev",  _matches)) => {},
        Some(("diagnose",  _matches)) => {},
        Some(("fix",  _matches)) => {},
        Some(("init",  _matches)) => {},
        Some(("options",  _matches)) => {},
        Some(("render",  _matches)) => {},
        Some(("run",  _matches)) => {},
        Some(("schema",  _matches)) => {},
        Some(("survey",  _matches)) => {},
        Some(("test",  _matches)) => {},
        Some(("version",  _matches)) => {},
        _ => unreachable!(),
    }
}
