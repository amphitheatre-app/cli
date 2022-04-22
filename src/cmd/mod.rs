use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("amp")
        .about("Amphitheatre's offcial command line tool")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Generate configuration for deploying an application")
                .arg(
                    Arg::new("analyze").long("analyze").takes_value(false).help(
                        "Print all discoverable Dockerfiles and images in JSON format to stdout",
                    ),
                )
                .arg(
                    Arg::new("filename").short('f').long("filename").default_value(".amp.yml").help(
                        "Path or URL to the Amphitheatre config file",
                    ),
                )
                .arg(Arg::new("force").long("force").takes_value(false).help(
                    "Force the generation of the Amphitheatre config",
                ))
                .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands)."),
        )
        .subcommand(Command::new("dev").about("Run a pipeline in development mode"))
        .subcommand(Command::new("run").about("Run a pipeline"))
        .subcommand(Command::new("debug").about("Run a pipeline in debug mode"))
        .subcommand(Command::new("clean").about("Remove any resources deployed by Amphitheatre"))
}
