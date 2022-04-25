use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("amp")
        .about("Amphitheatre's offcial command line tool")
        .arg_required_else_help(true)

        .subcommand(Command::new("apply")
            .about("Apply hydrated manifests to a cluster")
            .args(&[
                Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
                Arg::new("config").short('c').long("config").default_value("").help("File for global configurations (defaults to $HOME/.amp/config)"),
                Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
                Arg::new("force").long("force").takes_value(false).help("Recreate Kubernetes resources if necessary for deployment, warning: might cause downtime!"),
                Arg::new("iterative-status-check").long("iterative-status-check").takes_value(false)
                    .help("Run `status-check` iteratively after each deploy step, instead of all-together at the end of all deploys (default)"),
                Arg::new("kube-context").long("kube-context").default_value("").help("Deploy to this Kubernetes context"),
                Arg::new("kubeconfig").long("kubeconfig").default_value("").help("Path to the kubeconfig file to use for CLI requests"),
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
        )
        .subcommand(Command::new("build").about("Build the artifacts"))
        .subcommand(Command::new("clean").about("Delete any resources deployed by Amphitheatre"))
        .subcommand(Command::new("completion").about("Output shell completion for the given shell (bash or zsh)"))
        .subcommand(Command::new("config").about("Interact with the global Amphitheatre config file (defaults to $HOME/.amp/config)"))
        .subcommand(Command::new("debug").about("Run a pipeline in debug mode"))
        .subcommand(Command::new("deploy").about("Deploy pre-built artifacts"))
        .subcommand(Command::new("dev").about("Run a pipeline in development mode"))
        .subcommand(Command::new("diagnose").about("Run a diagnostic on Amphitheatre"))
        .subcommand(Command::new("fix").about("Update old configuration to a newer schema version"))

        .subcommand(Command::new("init")
            .about("Generate configuration for deploying an application")
            .args(&[
                Arg::new("analyze").long("analyze").takes_value(false).help("Print all discoverable Dockerfiles and images in JSON format to stdout"),
                Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
                Arg::new("force").long("force").takes_value(false).help("Force the generation of the Amphitheatre config")
            ])
            .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
        )

        .subcommand(Command::new("options").about("Outpu a list of global command-line options (applies to all commands)"))
        .subcommand(Command::new("render").about("Perform all image builds, and output rendered Kubernetes manifests"))
        .subcommand(Command::new("run").about("Run a pipeline"))
        .subcommand(Command::new("schema").about("List JSON schemas used to validate .amp.yaml configuration"))
        .subcommand(Command::new("survey").about("Opens a web browser to fill out the Amphitheatre survey"))
        .subcommand(Command::new("test").about("Run tests against your built application images"))
        .subcommand(Command::new("version").about("Print the version information"))
}
