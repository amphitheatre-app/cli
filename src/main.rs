mod cmd;

fn main() {
    let matches = cmd::build().get_matches();

    match matches.subcommand() {
        Some(("init",  _matches)) => {},
        Some(("dev",  _matches)) => {},
        Some(("run",  _matches)) => {},
        Some(("debug",  _matches)) => {},
        Some(("clean",  _matches)) => {},
        _ => unreachable!(),
    }
}
