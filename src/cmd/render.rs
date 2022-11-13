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
    Command::new("render")
        .about("Perform all image builds, and output rendered Kubernetes manifests")
        .args(&[
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("build-artifacts").short('a').long("build-artifacts").takes_value(true).help("File containing build result from a previous 'amp build --file-output'"),
            Arg::new("cache-artifacts").long("cache-artifacts").takes_value(false).help("Set to false to disable default caching of artifacts"),
            Arg::new("default-repo").short('d').long("default-repo").takes_value(true).help("Default repository value (overrides global config)"),
            Arg::new("digest-source").long("digest-source").default_value("remote")
                .help("Set to 'remote' to skip builds and resolve the digest of images by tag from \
                    the remote registry. Set to 'local' to build images locally and use digests from built \
                    images. Set to 'tag' to use tags directly from the build. Set to 'none' to use tags \
                    directly from the Kubernetes manifests."),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("hydration-dir").long("hydration-dir").default_value(".kpt-pipeline").help("The directory to where the (kpt) hydration takes place"),
            Arg::new("images").short('i').long("images").takes_value(true).help("A list of pre-built images to deploy, either tagged images or NAME=TAG pairs"),
            Arg::new("label").short('l').long("label").default_value("[]").help("Add custom labels to deployed objects. Set multiple times for multiple labels"),
            Arg::new("loud").long("loud").takes_value(false).help("Show the build logs and output"),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("namespace").short('n').long("namespace").takes_value(true).help("Run deployments in the specified namespace"),
            Arg::new("offline").long("offline").takes_value(false)
                .help("Do not connect to Kubernetes API server for manifest creation and validation. \
                    This is helpful when no Kubernetes cluster is available (e.g. GitOps model). \
                    No metadata.namespace attribute is injected in this case - the manifest content does not get changed."),
            Arg::new("output").short('o').long("output").takes_value(true).help("File to write the changed config (instead of standard output)"),
            Arg::new("profile").short('p').long("profile").default_value("[]").help("Activate profiles by name (prefixed with `-` to disable a profile)"),
            Arg::new("profile-auto-activation").long("profile-auto-activation").takes_value(false).help("Set to false to disable profile auto activation"),
            Arg::new("propagate-profiles").long("propagate-profiles").takes_value(false)
                .help("Setting '--propagate-profiles=false' disables propagating profiles set by the '--profile' \
                    flag across config dependencies. This mean that only profiles defined directly in the\
                    target '.amp.yaml' file are activated."),
            Arg::new("remote-cache-dir").long("remote-cache-dir").default_value("$HOME/.amp/repos").help("Specify the location of the git repositories cache"),
            Arg::new("resource-selector-rules-file").long("resource-selector-rules-file").takes_value(true)
                .help("Path to JSON file specifying the deny list of yaml objects for amp to \
                    NOT transform with 'image' and 'label' field replacements.  NOTE: this \
                    list is additive to amp's default denylist and denylist has priority over allowlist"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
                .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                to latest on access. `missing` only clones remote repositories if they do not exist \
                locally. `never` means the user takes responsibility for updating remote repositories."),
            Arg::new("wait-for-connection").long("wait-for-connection").takes_value(false).help("Blocks ending execution of amp until the /v2/events gRPC/HTTP endpoint is hit"),
        ])
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}

pub fn execute(args: &ArgMatches) {
    todo!()
}