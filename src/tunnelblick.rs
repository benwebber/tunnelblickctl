

use applescript;
use std::error::Error;


const TUNNELBLICK_SCRIPT: &'static str = include_str!("tunnelblick.applescript");


#[derive(Debug)]
pub enum Command {
    Connect(String),
    ConnectAll,
    Disconnect(String),
    DisconnectAll,
    GetConfigurations,
    GetStatus,
    GetVersion,
    Launch,
    Quit,
}

macro_rules! rpc_format {
    ($fn:expr) => {
        format!("tell Tunnelblick to {}()", $fn)
    };
    ($fn:expr, $arg:expr) => {
        format!("tell Tunnelblick to {}(\"{}\")", $fn, $arg)
    };
}

fn encode_command(command: Command) -> String {
    match command {
        Command::Connect(t) => rpc_format!("connect", t),
        Command::ConnectAll => rpc_format!("connectAll"),
        Command::Disconnect(t) => rpc_format!("disconnect", t),
        Command::DisconnectAll => rpc_format!("disconnectAll"),
        Command::GetConfigurations => rpc_format!("getConfigurations"),
        Command::GetStatus => rpc_format!("getStatus"),
        Command::GetVersion => rpc_format!("getVersion"),
        Command::Launch => rpc_format!("launch"),
        Command::Quit => rpc_format!("quit"),
    }
}

pub struct Tunnelblick {
    script: applescript::Script,
}

impl Tunnelblick {
    pub fn new() -> Tunnelblick {
        Tunnelblick {
            script: applescript::Script::new(TUNNELBLICK_SCRIPT),
        }
    }

    pub fn execute(&self, command: Command) -> Result<String, Box<Error>> {
        let mut script = self.script.clone();
        script.append(encode_command(command).as_ref());
        script.execute()
    }
}
