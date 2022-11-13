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

use clap::{Arg, ArgMatches, Command};

pub fn build() -> Command<'static> {
    Command::new("init")
        .about("Generate configuration for deploying an application")
        .args(&[
            Arg::new("analyze").long("analyze").takes_value(false).help("Print all discoverable Dockerfiles and images in JSON format to stdout"),
            Arg::new("artifact").short('a').long("artifact").takes_value(true)
                .help("'='-delimited Dockerfile/image pair, or JSON string, to generate build artifact\
                    (example: --artifact='{\"builder\":\"Docker\",\"payload\":{\"path\":\"/web/Dockerfile.web\"},\"image\":\"gcr.io/web-project/image\"}')"),
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("compose-file").long("compose-file").takes_value(true).help("Initialize from a docker-compose file"),
            Arg::new("default-kustomization").long("default-kustomization").takes_value(true).help("Default Kustomization overlay path (others will be added as profiles)"),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("force").long("force").takes_value(false).help("Force the generation of the Amphitheatre config"),
            Arg::new("generate-manifests").long("generate-manifests").takes_value(false)
                .help("Allows amp to try and generate basic kubernetes resources to get your project started"),
            Arg::new("kubernetes-manifest").short('k').long("kubernetes-manifest").takes_value(true)
                .help("A path or a glob pattern to kubernetes manifests (can be non-existent) to be added \
                    to the kubectl deployer (overrides detection of kubernetes manifests). Repeat the\
                    flag for multiple entries. E.g.: amp init -k pod.yaml -k k8s/*.yml"),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("remote-cache-dir").long("remote-cache-dir").default_value("$HOME/.amp/repos").help("Specify the location of the git repositories cache"),
            Arg::new("skip-build").long("skip-build").takes_value(false).help("Skip generating build artifacts in Amphitheatre config"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
            .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                to latest on access. `missing` only clones remote repositories if they do not exist \
                locally. `never` means the user takes responsibility for updating remote repositories."),
        ])
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}

pub fn execute(args: &ArgMatches) {
    todo!()
}