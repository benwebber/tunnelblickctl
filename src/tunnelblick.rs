use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use tabwriter::TabWriter;

pub struct Client {
    command: String,
    args: Vec<String>,
    script: String,
}

impl Client {
    pub fn command(command: &str) -> Client {
        Client {
            command: command.to_owned(),
            args: Vec::new(),
            script: include_str!("tunnelblick.applescript").to_owned(),
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Client {
        self.args.push(arg.to_owned());
        self
    }

    pub fn args(&mut self, args: &[&str]) -> &mut Client {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    fn compile_script(&self) -> String {
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
        return format!("{}\n{}", self.script, command);
    }

    pub fn send(&self) -> String {
        let script = self.compile_script();

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
                return String::from_utf8(tw.unwrap()).unwrap();
            }
            _ => return s,
        }
    }
}
