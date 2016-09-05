#[macro_use]
extern crate clap;
extern crate tabwriter;

mod cli;
mod tunnelblick;

fn version() -> String {
    let cli_version = crate_version!();
    let command = tunnelblick::cmd("getVersion");
    let app_version = tunnelblick::Client::new().send(&command);
    return format!("{} {}\nTunnelblick {}",
                   env!("CARGO_PKG_NAME"),
                   cli_version,
                   app_version);
}

fn main() {
    let matches = cli::cli().get_matches();

    if matches.is_present("version") {
        print!("{}", version());
        return;
    }

    let mut cmd = tunnelblick::Cmd::new();
    match matches.subcommand() {
        ("connect", Some(m)) => cmd.cmd("connect").arg(m.value_of("name").unwrap()),
        ("disconnect", Some(m)) => cmd.cmd("disconnect").arg(m.value_of("name").unwrap()),
        ("list", Some(_)) => cmd.cmd("listTunnels"),
        ("status", Some(_)) => cmd.cmd("showStatus"),
        ("quit", Some(_)) => cmd.cmd("quit"),
        ("launch", Some(_)) => cmd.cmd("run"),
        // Should never reach here.
        _ => panic!("cannot match command"),
    };
    let client = tunnelblick::Client::new();
    let output = client.send(&cmd);
    print!("{}", output);
}
