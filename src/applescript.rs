use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub struct Script {
    script: String,
}

impl Script {
    pub fn new(script: &str) -> Script {
        Script { script: script.to_owned() }
    }

    pub fn append(&mut self, script: &str) -> &mut Script {
        self.script.push_str(script);
        self
    }

    pub fn execute(&self) -> Result<String, Box<Error>> {
        let process = try!(Command::new("osascript")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn());

        try!(process.stdin.unwrap().write_all(self.script.as_bytes()));

        let mut s = String::new();
        try!(process.stdout.unwrap().read_to_string(&mut s));

        return Ok(s);
    }
}
