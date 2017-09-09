use applescript::{self, AppleScriptCommand};
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

impl AppleScriptCommand for Command {
    fn as_rpc_command(&self, script: &str) -> String {
        use self::Command::*;
        match self {
            &Connect(ref t) => rpc_format!(script, "connect", t),
            &ConnectAll => rpc_format!(script, "connectAll"),
            &Disconnect(ref t) => rpc_format!(script, "disconnect", t),
            &DisconnectAll => rpc_format!(script, "disconnectAll"),
            &GetConfigurations => rpc_format!(script, "getConfigurations"),
            &GetStatus => rpc_format!(script, "getStatus"),
            &GetVersion => rpc_format!(script, "getVersion"),
            &Launch => rpc_format!(script, "launch"),
            &Quit => rpc_format!(script, "quit"),
        }
    }
}

pub struct Tunnelblick {
    script: applescript::Script,
}

impl Tunnelblick {
    pub fn new() -> Tunnelblick {
        Tunnelblick { script: applescript::Script::new(TUNNELBLICK_SCRIPT) }
    }

    pub fn execute(&self, command: Command) -> Result<String, Box<Error>> {
        let mut script = self.script.clone();
        script.append(command.as_rpc_command("Tunnelblick").as_ref());
        script.execute()
    }
}
