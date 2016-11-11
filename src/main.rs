
use std::error::Error;use std::io::Write;

#[macro_use]
extern crate clap;
extern crate tabwriter;

use clap::App;
use tabwriter::TabWriter;

mod applescript;
mod tunnelblick;

const TUNNELBLICK_SCRIPT: &'static str = include_str!("tunnelblick.applescript");

fn complete(shell: &str) -> &'static str {
    return match shell {
        _ => include_str!("../contrib/tunnelblick.bash"),
    };
}

fn version() -> Result<String, Box<Error>> {
    let cli_version = option_env!("VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let command = tunnelblick::cmd("getVersion");
    let mut script = applescript::Script::new(TUNNELBLICK_SCRIPT);
    script.append(command.encode().as_ref());
    let app_version = try!(script.execute());
    return Ok(format!("{} {}\nTunnelblick {}",
                      env!("CARGO_PKG_NAME"),
                      cli_version,
                      app_version));
}

fn main() {
    let spec = load_yaml!("cli.yaml");
    let matches = App::from_yaml(spec).get_matches();

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
        ("connect", Some(m)) => {
            if m.is_present("all") {
                cmd.cmd("connectAll")
            } else {
                cmd.cmd("connect").arg(m.value_of("VPN").unwrap())
            }
        }
        ("disconnect", Some(m)) => {
            if m.is_present("all") {
                cmd.cmd("disconnectAll")
            } else {
                cmd.cmd("disconnect").arg(m.value_of("VPN").unwrap())
            }
        }
        ("list", Some(_)) => cmd.cmd("getConfigurations"),
        ("status", Some(_)) => cmd.cmd("getStatus"),
        ("quit", Some(_)) => cmd.cmd("quit"),
        ("launch", Some(_)) => cmd.cmd("run"),
        // Should never reach here.
        _ => panic!("cannot match command"),
    };
    let mut script = applescript::Script::new(TUNNELBLICK_SCRIPT);
    script.append(cmd.encode().as_ref());
    let output = script.execute();

    match output {
        Err(why) => panic!(why.to_string()),
        Ok(v) => {
            if matches.is_present("status") {
                let mut tw = TabWriter::new(Vec::new());
                tw.write(v.as_bytes()).unwrap();
                tw.flush().unwrap();
                print!("{}", String::from_utf8(tw.unwrap()).unwrap());
            } else {
                print!("{}", v);
            }
        }
    }
}
