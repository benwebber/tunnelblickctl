#[macro_use]
extern crate clap;
extern crate tabwriter;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use tabwriter::TabWriter;


use clap::{App, AppSettings, Arg, SubCommand};

struct Tunnelblick {
    command: String,
    args: Vec<String>,
    client: String,
}

impl Tunnelblick {
    fn new(command: &str) -> Tunnelblick {
        Tunnelblick {
            command: command.to_owned(),
            args: Vec::new(),
            client: include_str!("tunnelblick.applescript").to_owned(),
        }
    }

    fn arg(&mut self, arg: &str) -> &mut Tunnelblick {
        self.args.push(arg.to_owned());
        self
    }

    fn args(&mut self, args: &[&str]) -> &mut Tunnelblick {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    fn spawn(&self) {
        let command = match self.command.as_ref() {
            "run" => String::from("run Tunnelblick"),
            _ => {
                format!("tell Tunnelblick to {}({})",
                        self.command,
                        // Quote all arguments when rendering script.
                        self.args
                            .iter()
                            .map(|arg| format!("{:?}", arg))
                            .collect::<Vec<String>>()
                            .join(","))
            }
        };

        let script = self.client.clone() + &command;

        let process = match Command::new("osascript")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Err(why) => panic!("couldn't spawn osascript: {}", why.description()),
            Ok(process) => process,
        };


        match process.stdin.unwrap().write_all(script.as_bytes()) {
            Err(why) => panic!("couldn't write to osascript stdin: {}", why.description()),
            Ok(_) => {}
        }

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read osascript stdout: {}", why.description()),
            Ok(_) => {}
        }

        match self.command.as_ref() {
            "showStatus" => {
                let mut tw = TabWriter::new(Vec::new());
                tw.write(s.as_bytes()).unwrap();
                tw.flush().unwrap();
                print!("{}", String::from_utf8(tw.unwrap()).unwrap());
            }
            _ => {
                print!("{}", s);
            }
        }
    }
}

fn main() {



    let mut app = App::new("tunnelblickctl")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("connect")
            .about("Connect to a VPN")
            .arg(Arg::with_name("name")
                .help("VPN to connect to")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("disconnect")
            .about("Disconnect from a VPN")
            .arg(Arg::with_name("name")
                .help("VPN to disconnect from")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("list")
            .aliases(&["ls"])
            .about("List VPN configurations"))
        .subcommand(SubCommand::with_name("start").about("Start Tunnelblick"))
        .subcommand(SubCommand::with_name("status").about("Show VPN connection status"))
        .subcommand(SubCommand::with_name("quit")
            .aliases(&["stop"])
            .about("Quit Tunnelblick"));

    // Do not consume App with App::get_matches(). Allows us to use
    // App::print_help() below.
    let matches = app.get_matches_from_safe_borrow(env::args()).unwrap_or_else(|e| e.exit());;

    match matches.subcommand() {
        ("connect", Some(m)) => {
            Tunnelblick::new("connect").arg(m.value_of("name").unwrap()).spawn()
        }
        ("disconnect", Some(m)) => {
            Tunnelblick::new("disconnect").arg(m.value_of("name").unwrap()).spawn()
        }
        ("list", Some(_)) => Tunnelblick::new("listTunnels").spawn(),
        ("status", Some(_)) => Tunnelblick::new("showStatus").spawn(),
        ("quit", Some(_)) => Tunnelblick::new("quit").spawn(),
        ("start", Some(_)) => Tunnelblick::new("run").spawn(),
        _ => {}
    }
}
