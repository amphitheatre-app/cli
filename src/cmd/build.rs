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
    Command::new("build")
        .about("Build the artifacts")
        .args(&[
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("build-concurrency").long("build-concurrency").default_value("-1]")
                .help("Number of concurrently running builds. Set to 0 to run all builds in parallel. Doesn't violate build order among dependencies."),
            Arg::new("build-image").short('b').long("build-image").default_value("[]")
                .help("Only build artifacts with image names that contain the given substring.\
                    Default is to build sources for all artifacts --cache-artifacts=true: \
                    Set to false to disable default caching of artifacts"),
            Arg::new("cache-file").long("cache-file").default_value("$HOME/.amp/cache").help("Specify the location of the cache file"),
            Arg::new("config").short('c').long("config").default_value("$HOME/.amp/config").help("File for global configurations"),
            Arg::new("default-repo").short('d').long("default-repo").takes_value(true).help("Default repository value (overrides global config)"),
            Arg::new("detect-minikube").long("detect-minikube").takes_value(false).help("Use heuristics to detect a minikube cluster"),
            Arg::new("dry-run").long("dry-run").takes_value(false).help("Don't build images, just compute the tag for each artifact."),
            Arg::new("file-output").long("file-output").takes_value(true).help("Filename to write build images to"),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("insecure-registry").long("insecure-registry").default_value("[]").help("Target registries for built images which are not secure"),
            Arg::new("kube-context").long("kube-context").takes_value(true).help("Deploy to this Kubernetes context"),
            Arg::new("kubeconfig").long("kubeconfig").takes_value(true).help("Path to the kubeconfig file to use for CLI requests"),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("mute-logs").long("mute-logs").default_value("[]").help("Mute logs for specified stages in pipeline (build, deploy, status-check, none, all)"),
            Arg::new("namespace").short('n').long("namespace").takes_value(true).help("Run deployments in the specified namespace"),
            Arg::new("output").short('o').long("output").default_value("{{json .}}").help("Used in conjunction with --quiet flag"),
            Arg::new("platform").long("platform").default_value("[]").help("The platform to target for the build artifacts"),
            Arg::new("profile").short('p').long("profile").default_value("[]").help("Activate profiles by name (prefixed with `-` to disable a profile)"),
            Arg::new("profile-auto-activation").long("profile-auto-activation").takes_value(false).help("Set to false to disable profile auto activation"),
            Arg::new("propagate-profiles").long("propagate-profiles").takes_value(false)
                .help("Setting '--propagate-profiles=false' disables propagating profiles set by the '--profile' \
                    flag across config dependencies. This mean that only profiles defined directly in the\
                    target '.amp.yaml' file are activated."),
            Arg::new("push").long("push").takes_value(false).help("Push the built images to the specified image repository"),
            Arg::new("quiet").short('q').long("quiet").help("Suppress the build output and print image built on success. See --output to format output."),
            Arg::new("remote-cache-dir").long("remote-cache-dir").default_value("$HOME/.amp/repos").help("Specify the location of the git repositories cache"),
            Arg::new("rpc-http-port").long("rpc-http-port").takes_value(true).help("tcp port to expose the Amphitheatre API over HTTP REST"),
            Arg::new("rpc-port").long("rpc-port").takes_value(true).help("tcp port to expose the Amphitheatre API over gRPC"),
            Arg::new("skip-tests").long("skip-tests").takes_value(false).help("Whether to skip the tests after building"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
            .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                to latest on access. `missing` only clones remote repositories if they do not exist \
                locally. `never` means the user takes responsibility for updating remote repositories."),
            Arg::new("tag").short('t').long("tag").takes_value(true)
                .help("The optional custom tag to use for images which overrides the current Tagger configuration"),
            Arg::new("toot").long("toot").takes_value(false).help("Emit a terminal beep after the deploy is complete"),
            Arg::new("wait-for-connection").long("wait-for-connection").takes_value(false).help("Blocks ending execution of amp until the /v2/events gRPC/HTTP endpoint is hit"),
        ])
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}

pub fn execute(args: &ArgMatches) {
    todo!()
}