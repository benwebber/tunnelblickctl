use std::error;
use std::fmt;

use serde::{Deserialize, Serialize};

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
    #[serde(rename = "bytesOut")]
    pub bytes_out: u64,
    #[serde(rename = "bytesIn")]
    pub bytes_in: u64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum Response {
    #[serde(rename = "success")]
    Success(ResponseData),
    #[serde(rename = "error")]
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
    message: String,
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
            Connect(name) => rpc_format!("connect", name),
            ConnectAll => rpc_format!("connectAll"),
            Disconnect(name) => rpc_format!("disconnect", name),
            DisconnectAll => rpc_format!("disconnectAll"),
            GetStatus => rpc_format!("getStatus"),
            GetVersion => rpc_format!("getVersion"),
            Install(name) => rpc_format!("install", name),
            Launch => rpc_format!("launch"),
            List => rpc_format!("list"),
            Quit => rpc_format!("quit"),
        };
        let script = include_str!(concat!(env!("OUT_DIR"), "/tunnelblick.js"));
        write!(f, "{}{}", script, call)
    }
}

impl Command {
    pub fn execute_with<F>(&self, runner: F) -> Result<ResponseData, Box<dyn error::Error>>
    where
        F: FnOnce(&str) -> Result<Response, Box<dyn error::Error>>,
    {
        let response = runner(&format!("{}", self))?;
        match response {
            Response::Success(data) => Ok(data),
            Response::Error(message) => Err(Box::new(Error { message })),
        }
    }

    pub fn execute(&self) -> Result<ResponseData, Box<dyn error::Error>> {
        self.execute_with(|script| {
            osascript::JavaScript::new(script)
                .execute::<Response>()
                .map_err(Into::into)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_success_string() {
        let json = r#"{"status":"success","data":"5.3.4"}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert!(matches!(r, Response::Success(ResponseData::String(_))));
    }

    #[test]
    fn response_success_string_array() {
        let json = r#"{"status":"success","data":["vpn-a","vpn-b"]}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert!(matches!(r, Response::Success(ResponseData::StringArray(_))));
    }

    #[test]
    fn response_success_configurations() {
        let json = r#"{"status":"success","data":[{"name":"vpn","state":"CONNECTED","autoconnect":"yes","bytesIn":1024,"bytesOut":2048}]}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert!(matches!(r, Response::Success(ResponseData::Configurations(_))));
    }

    #[test]
    fn response_error() {
        let json = r#"{"status":"error","data":"Tunnelblick is not running"}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert!(matches!(r, Response::Error(_)));
    }

    macro_rules! test_response_data {
        ($name:ident, $json:expr, $pat:pat) => {
            #[test]
            fn $name() {
                let r: ResponseData = serde_json::from_str($json).unwrap();
                assert!(matches!(r, $pat));
            }
        };
    }

    test_response_data!(
        response_data_configurations,
        r#"[{"name":"vpn","state":"CONNECTED","autoconnect":"yes","bytesIn":1024,"bytesOut":2048}]"#,
        ResponseData::Configurations(_)
    );
    test_response_data!(response_data_string_array, r#"["a","b"]"#, ResponseData::StringArray(_));
    test_response_data!(response_data_string, r#""hello""#, ResponseData::String(_));
    test_response_data!(response_data_boolean, "true", ResponseData::Boolean(_));
    test_response_data!(response_data_integer, "42", ResponseData::Integer(_));

    #[test]
    fn execute_with_get_version_success() {
        let result = Command::GetVersion.execute_with(|_| {
            Ok(Response::Success(ResponseData::String("5.3.4".to_string())))
        });
        assert!(matches!(result, Ok(ResponseData::String(_))));
    }

    #[test]
    fn execute_with_list_success() {
        let result = Command::List.execute_with(|_| {
            Ok(Response::Success(ResponseData::StringArray(vec![
                "vpn-a".to_string(),
                "vpn-b".to_string(),
            ])))
        });
        assert!(matches!(result, Ok(ResponseData::StringArray(_))));
    }

    #[test]
    fn execute_with_get_status_success() {
        let result = Command::GetStatus.execute_with(|_| {
            Ok(Response::Success(ResponseData::Configurations(vec![
                Configuration {
                    name: "vpn".to_string(),
                    state: "CONNECTED".to_string(),
                    autoconnect: "yes".to_string(),
                    bytes_in: 1024,
                    bytes_out: 2048,
                },
            ])))
        });
        assert!(matches!(result, Ok(ResponseData::Configurations(_))));
    }

    #[test]
    fn execute_with_error_response() {
        let result = Command::GetVersion
            .execute_with(|_| Ok(Response::Error("Tunnelblick is not running".to_string())));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Tunnelblick is not running");
    }

    #[test]
    fn execute_with_runner_error() {
        let result = Command::GetVersion
            .execute_with(|_| -> Result<Response, Box<dyn std::error::Error>> {
                Err("runner failed".into())
            });
        assert!(result.is_err());
    }

    macro_rules! test_command_display {
        ($name:ident, $cmd:expr, $suffix:expr) => {
            #[test]
            fn $name() {
                assert!(format!("{}", $cmd).ends_with($suffix));
            }
        };
    }

    test_command_display!(
        display_connect,
        Command::Connect("work-vpn".to_string()),
        r#"return rpc.call("connect", "work-vpn");"#
    );
    test_command_display!(
        display_connect_all,
        Command::ConnectAll,
        r#"return rpc.call("connectAll");"#
    );
    test_command_display!(
        display_disconnect,
        Command::Disconnect("work-vpn".to_string()),
        r#"return rpc.call("disconnect", "work-vpn");"#
    );
    test_command_display!(
        display_disconnect_all,
        Command::DisconnectAll,
        r#"return rpc.call("disconnectAll");"#
    );
    test_command_display!(
        display_get_status,
        Command::GetStatus,
        r#"return rpc.call("getStatus");"#
    );
    test_command_display!(
        display_get_version,
        Command::GetVersion,
        r#"return rpc.call("getVersion");"#
    );
    test_command_display!(
        display_install,
        Command::Install("/path/to/vpn.tblk".to_string()),
        r#"return rpc.call("install", "/path/to/vpn.tblk");"#
    );
    test_command_display!(display_launch, Command::Launch, r#"return rpc.call("launch");"#);
    test_command_display!(display_list, Command::List, r#"return rpc.call("list");"#);
    test_command_display!(display_quit, Command::Quit, r#"return rpc.call("quit");"#);
}
