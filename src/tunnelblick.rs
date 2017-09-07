

use applescript;
use std::error::Error;


const TUNNELBLICK_SCRIPT: &'static str = include_str!("tunnelblick.applescript");


pub struct Cmd {
    name: String,
    args: Vec<String>,
}


impl Cmd {
    pub fn new() -> Cmd {
        Cmd {
            name: String::new(),
            args: Vec::new(),
        }
    }

    pub fn cmd(&mut self, name: &str) -> &mut Cmd {
        self.name = name.to_owned();
        self
    }

    pub fn arg(&mut self, arg: &str) -> &mut Cmd {
        self.args.push(arg.to_owned());
        self
    }

    pub fn args(&mut self, args: &[&str]) -> &mut Cmd {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn encode(&self) -> String {
        return match self.name.as_ref() {
            "run" => String::from("run Tunnelblick"),
            _ => {
                format!("tell Tunnelblick to {}({})",
                        self.name,
                        // Quote all arguments when rendering script.
                        self.args
                            .iter()
                            .map(|arg| format!("{:?}", arg))
                            .collect::<Vec<String>>()
                            .join(","))
            }
        };
    }

    pub fn execute(&self) -> Result<String, Box<Error>> {
        let mut script = applescript::Script::new(TUNNELBLICK_SCRIPT);
        script.append(self.encode().as_ref());
        script.execute()
    }
}

pub fn cmd(name: &str) -> Cmd {
    let mut cmd = Cmd::new();
    cmd.cmd(name);
    cmd
}
