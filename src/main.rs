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
