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

use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("apply")
        .about("Apply hydrated manifests to a cluster")
        .args(&[
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("config").short('c').long("config").default_value("$HOME/.amp/config").help("File for global configurations"),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("force").long("force").takes_value(false).help("Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!"),
            Arg::new("iterative-status-check").long("iterative-status-check").takes_value(false)
                .help("Run `status-check` iteratively after each deploy step, instead of all-together at the end of all deploys (default)"),
            Arg::new("kube-context").long("kube-context").takes_value(true).help("Deploy to this Kubernetes context"),
            Arg::new("kubeconfig").long("kubeconfig").takes_value(true).help("Path to the kubeconfig file to use for CLI requests"),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("namespace").short('n').long("namespace").default_value("").help("Run deployments in the specified namespace"),
            Arg::new("profile").short('p').long("profile").default_value("[]").help("Activate profiles by name (prefixed with `-` to disable a profile)"),
            Arg::new("remote-cache-dir").long("remote-cache-dir").default_value("[]").help("Specify the location of the git repositories cache (default $HOME/.amp/repos)"),
            Arg::new("status-check").long("status-check").takes_value(false).help("Wait for deployed resources to stabilize"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
                .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                    One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                    to latest on access. `missing` only clones remote repositories if they do not exist \
                    locally. `never` means the user takes responsibility for updating remote repositories."),
            Arg::new("tail").long("tail").takes_value(false).help("Stream logs from deployed objects"),
            Arg::new("wait-for-connection").long("wait-for-connection").takes_value(false).help("Blocks ending execution of amp until the /v2/events gRPC/HTTP endpoint is hit"),
        ])
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}
