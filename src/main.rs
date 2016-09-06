use std::error::Error;

#[macro_use]
extern crate clap;
extern crate tabwriter;

mod cli;
mod tunnelblick;

fn complete(shell: &str) -> &'static str {
    return match shell {
        _ => include_str!("../contrib/tunnelblick.bash"),
    }
}

fn version() -> Result<String, Box<Error>> {
    let cli_version = crate_version!();
    let command = tunnelblick::cmd("getVersion");
    let app_version = try!(tunnelblick::Client::new().send(&command));
    return Ok(format!("{} {}\nTunnelblick {}",
                      env!("CARGO_PKG_NAME"),
                      cli_version,
                      app_version));
}

fn main() {
    let matches = cli::cli().get_matches();

    if matches.is_present("version") {
        let version = match version() {
            Err(why) => panic!(why.to_string()),
            Ok(v) => v,
        };
        print!("{}", version);
        return;
    }

    if matches.is_present("complete") {
        print!("{}", complete("bash"));
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

    match output {
        Err(why) => panic!(why.to_string()),
        Ok(v) => print!("{}", v),
    }
}
