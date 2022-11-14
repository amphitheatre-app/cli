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
use errors::Result;

pub fn build() -> Command<'static> {
    Command::new("deploy")
        .about("Deploy pre-built artifacts")
        .args(&[
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("build-artifacts").short('a').long("build-artifacts").takes_value(true).help("File containing build result from a previous 'amp build --file-output'"),
            Arg::new("build-concurrency").long("build-concurrency").default_value("-1]")
                .help("Number of concurrently running builds. Set to 0 to run all builds in parallel. Doesn't violate build order among dependencies."),
            Arg::new("config").short('c').long("config").default_value("$HOME/.amp/config").help("File for global configurations"),
            Arg::new("default-repo").short('d').long("default-repo").takes_value(true).help("Default repository value (overrides global config)"),
            Arg::new("detect-minikube").long("detect-minikube").takes_value(false).help("Use heuristics to detect a minikube cluster"),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("force").long("force").takes_value(false).help("Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!"),
            Arg::new("hydration-dir").long("hydration-dir").default_value(".kpt-pipeline").help("The directory to where the (kpt) hydration takes place"),
            Arg::new("images").short('i').long("images").takes_value(true).help("A list of pre-built images to deploy, either tagged images or NAME=TAG pairs"),
            Arg::new("iterative-status-check").long("iterative-status-check").takes_value(false)
                .help("Run `status-check` iteratively after each deploy step, instead of all-together at the end of all deploys (default)"),
            Arg::new("kube-context").long("kube-context").takes_value(true).help("Deploy to this Kubernetes context"),
            Arg::new("kubeconfig").long("kubeconfig").takes_value(true).help("Path to the kubeconfig file to use for CLI requests"),
            Arg::new("label").short('l').long("label").default_value("[]").help("Add custom labels to deployed objects. Set multiple times for multiple labels"),
            Arg::new("load-images").long("load-images").takes_value(false).help("If true, amp will force load the container images into the local cluster."),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("mute-logs").long("mute-logs").default_value("[]").help("Mute logs for specified stages in pipeline (build, deploy, status-check, none, all)"),
            Arg::new("namespace").short('n').long("namespace").takes_value(true).help("Run deployments in the specified namespace"),
            Arg::new("platform").long("platform").default_value("[]").help("The platform to target for the build artifacts"),
            Arg::new("port-forward").long("port-forward").default_value("off")
                .help("Port-forward exposes service ports and container ports within pods and other resources (off, user, services, debug, pods)"),
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
            Arg::new("rpc-http-port").long("rpc-http-port").takes_value(true).help("tcp port to expose the Amphitheatre API over HTTP REST"),
            Arg::new("rpc-port").long("rpc-port").takes_value(true).help("tcp port to expose the Amphitheatre API over gRPC"),
            Arg::new("skip-render").long("skip-render").takes_value(false).help("Don't render the manifests, just deploy them"),
            Arg::new("status-check").long("status-check").takes_value(false).help("Wait for deployed resources to stabilize"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
            .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                to latest on access. `missing` only clones remote repositories if they do not exist \
                locally. `never` means the user takes responsibility for updating remote repositories."),
            Arg::new("tag").short('t').long("tag").takes_value(true)
                .help("The optional custom tag to use for images which overrides the current Tagger configuration"),
            Arg::new("tail").long("tail").takes_value(false).help("Stream logs from deployed objects"),
            Arg::new("toot").long("toot").takes_value(false).help("Emit a terminal beep after the deploy is complete"),
            Arg::new("wait-for-connection").long("wait-for-connection").takes_value(false).help("Blocks ending execution of amp until the /v2/events gRPC/HTTP endpoint is hit"),
            Arg::new("wait-for-deletions").long("wait-for-deletions").takes_value(false).help("Wait for pending deletions to complete before a deployment"),
            Arg::new("wait-for-deletions-delay").long("wait-for-deletions-delay").default_value("2s")
                .help("Delay between two checks for pending deletions"),
            Arg::new("wait-for-deletions-max").long("wait-for-deletions-max").default_value("1m0s")
                .help("Max duration to wait for pending deletions"),
        ])
        .after_help(super::AFTER_HELP_STRING)
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    todo!()
}