use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("version")
        .about("Print the version information")
        .arg(Arg::new("output").short('o').long("output").default_value("{{.Version}}").help("Format output"))
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}
