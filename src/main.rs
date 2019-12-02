use std::fs;
use std::error::Error;
use std::io;
use std::path::PathBuf;

#[macro_use]
extern crate clap;
extern crate csv;
extern crate humansize;
extern crate osascript;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tabwriter;

use clap::App;
use humansize::{FileSize, file_size_opts};
use tabwriter::TabWriter;

#[macro_use]
mod tunnelblick;


#[derive(Debug, Deserialize, Serialize)]
pub struct HumanConfiguration {
    #[serde(rename = "NAME")]
    name: String,
    #[serde(rename = "STATE")]
    state: String,
    #[serde(rename = "AUTOCONNECT")]
    autoconnect: String,
    #[serde(rename = "TX")]
    bytes_out: String,
    #[serde(rename = "RX")]
    bytes_in: String,
}

impl From<&tunnelblick::Configuration> for HumanConfiguration {
    fn from(config: &tunnelblick::Configuration) -> Self {
        return Self {
            autoconnect: config.autoconnect.clone(),
            state: config.state.clone(),
            name: config.name.clone(),
            bytes_in: config.bytes_in.file_size(file_size_opts::BINARY).unwrap(),
            bytes_out: config.bytes_out.file_size(file_size_opts::BINARY).unwrap(),
        }
    }
}


fn complete(shell: &str) -> &'static str {
    return match shell {
        _ => include_str!("../contrib/tunnelblick.bash"),
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let version = option_env!("version").unwrap_or(env!("CARGO_PKG_VERSION"));
    let spec = load_yaml!("cli.yaml");
    let matches = App::from_yaml(spec).get_matches();

    if matches.is_present("complete") {
        print!("{}", complete("bash"));
        return Ok(());
    }

    let command = match matches.subcommand() {
        ("connect", Some(m)) => {
            if m.is_present("all") {
                tunnelblick::Command::ConnectAll
            } else {
                tunnelblick::Command::Connect(m.value_of("VPN").unwrap().to_string())
            }
        }
        ("disconnect", Some(m)) => {
            if m.is_present("all") {
                tunnelblick::Command::DisconnectAll
            } else {
                tunnelblick::Command::Disconnect(m.value_of("VPN").unwrap().to_string())
            }
        }
        ("install", Some(m)) => {
            let path = PathBuf::from(m.value_of("FILE").unwrap().to_string());
            let absolute_path = fs::canonicalize(&path);
            tunnelblick::Command::Install(absolute_path.unwrap().to_str().unwrap().to_string())
        },
        ("list", Some(_)) => tunnelblick::Command::List,
        ("status", Some(_)) => {
            tunnelblick::Command::GetStatus
        },
        ("quit", Some(_)) => tunnelblick::Command::Quit,
        ("launch", Some(_)) => tunnelblick::Command::Launch,
        ("version", Some(_)) => tunnelblick::Command::GetVersion,
        _ => unreachable!(),
    };

    let bytes = match matches.subcommand() {
        ("status", Some(matches)) => {
            matches.is_present("bytes")
        },
        _ => false,
    };

    let response = command.execute();

    match (command, response) {
        (tunnelblick::Command::List, Ok(data)) => {
            match data {
                tunnelblick::ResponseData::StringArray(configs) => {
                    for config in configs.iter() {
                        println!("{}", config);
                    }
                },
                _ => unreachable!(),
            }
        },
        (tunnelblick::Command::GetStatus, Ok(data)) => {
            match data {
                tunnelblick::ResponseData::Configurations(configs) => {
                    let tab_writer = TabWriter::new(io::stdout());
                    let mut writer = csv::WriterBuilder::new().has_headers(false).delimiter(b'\t').from_writer(tab_writer);
                    writer.write_record(&["NAME", "STATE", "AUTOCONNECT", "TX", "RX"])?;
                    for config in configs.iter() {
                        if bytes {
                            writer.serialize(config)?;
                        } else {
                            let human_config = HumanConfiguration::from(config);
                            writer.serialize(human_config)?;
                        }
                    }
                },
                _ => unreachable!(),
            }
        },
        (tunnelblick::Command::GetVersion, Ok(data)) => {
            println!("{} {}\nTunnelblick {}", env!("CARGO_PKG_NAME"), version, data);
        },
        (tunnelblick::Command::GetVersion, Err(_)) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), version);
        },
        (_, Err(message)) => {
            return Err(message);
        }
        _ => (),
    }

    Ok(())
}
