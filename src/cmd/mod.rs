use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("amp")
        .about("Amphitheatre's offcial command line tool")
        .arg_required_else_help(true)

        .subcommand(Command::new("apply").about("Apply hydrated manifests to a cluster"))
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
            .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands)."),
        )

        .subcommand(Command::new("options").about("Outpu a list of global command-line options (applies to all commands)"))
        .subcommand(Command::new("render").about("Perform all image builds, and output rendered Kubernetes manifests"))
        .subcommand(Command::new("run").about("Run a pipeline"))
        .subcommand(Command::new("schema").about("List JSON schemas used to validate .amp.yaml configuration"))
        .subcommand(Command::new("survey").about("Opens a web browser to fill out the Amphitheatre survey"))
        .subcommand(Command::new("test").about("Run tests against your built application images"))
        .subcommand(Command::new("version").about("Print the version information"))
}
