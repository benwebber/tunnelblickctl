use std::error;
use std::fmt;

use osascript;

#[derive(Debug)]
pub enum Command {
    Connect(String),
    ConnectAll,
    Disconnect(String),
    DisconnectAll,
    List,
    GetStatus,
    GetVersion,
    Install(String),
    Launch,
    Quit,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    // The order defined here corresponds to the order in `status` output.
    // TODO: Define a separate display struct.
    pub name: String,
    pub state: String,
    pub autoconnect: String,
    #[serde(rename="bytesOut")]
    pub bytes_out: u64,
    #[serde(rename="bytesIn")]
    pub bytes_in: u64,
}


#[derive(Debug, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum Response {
    #[serde(rename="success")]
    Success(ResponseData),
    #[serde(rename="error")]
    Error(String),
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseData {
    Configurations(Vec<Configuration>),
    StringArray(Vec<String>),
    String(String),
    Boolean(bool),
    Integer(i64),
}


impl fmt::Display for ResponseData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Configurations(v) => write!(f, "{:?}", v),
            Self::StringArray(v) => write!(f, "{:?}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Boolean(v) => write!(f, "{}", v),
            Self::Integer(v) => write!(f, "{}", v),
        }
    }
}

pub struct Error {
    message: String
}

// main() prints errors with debug formatting. Use the default string format instead of `Error { ... }`.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {}

macro_rules! rpc_format {
    ($fn:expr) => {
        format!("return rpc.call(\"{}\");", $fn)
    };
    ($fn:expr, $arg:expr) => {
        format!("return rpc.call(\"{}\", \"{}\");", $fn, $arg)
    };
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Command::*;
        let call = match self {
            Connect(ref name) => rpc_format!("connect", name),
            ConnectAll => rpc_format!("connectAll"),
            Disconnect(ref name) => rpc_format!("disconnect", name),
            DisconnectAll => rpc_format!("disconnectAll"),
            GetStatus => rpc_format!("getStatus"),
            GetVersion => rpc_format!("getVersion"),
            Install(ref name) => rpc_format!("install", name),
            Launch => rpc_format!("launch"),
            List => rpc_format!("list"),
            Quit => rpc_format!("quit"),
        };
        let script = include_str!("tunnelblick.js");
        write!(f, "{}{}", script, call)
    }
}

impl Command {
    pub fn execute(&self) -> Result<ResponseData, Box<dyn error::Error>> {
        let script = osascript::JavaScript::new(&format!("{}", self));
        let response: Response = script.execute()?;
        match response {
            Response::Success(data) => Ok(data),
            Response::Error(message) => Err(Box::new(Error {message: message})),
        }
    }
}
