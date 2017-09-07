use std::error::Error;
use std::fmt;
use std::io::{self, Read, Write};
use std::str::FromStr;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tabwriter;

use clap::App;
use csv::ReaderBuilder;
use serde::de::{self, Deserialize, Deserializer};
use tabwriter::TabWriter;

mod applescript;
mod tunnelblick;


#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    autoconnect: String,
    state: String,
    name: String,
    #[serde(deserialize_with = "from_str")]
    bytesin: u64,
    #[serde(deserialize_with = "from_str")]
    bytesout: u64,
}


fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: fmt::Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}


fn complete(shell: &str) -> &'static str {
    return match shell {
        _ => include_str!("../contrib/tunnelblick.bash"),
    };
}

fn version() -> Result<String, Box<Error>> {
    let cli_version = option_env!("VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let command = tunnelblick::cmd("getVersion");
    let app_version = try!(command.execute());
    return Ok(format!("{} {}\nTunnelblick {}\n",
                      env!("CARGO_PKG_NAME"),
                      cli_version,
                      app_version));
}


fn print_status<R: Read>(mut reader: csv::Reader<R>) -> Result<(), Box<Error>> {
    let mut tab_writer = TabWriter::new(io::stdout());
    let mut csv_writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(tab_writer);
    for record in reader.deserialize() {
        let config: Configuration = record?;
        csv_writer.serialize(config);
    }
    Ok(())
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

    let output = cmd.execute();

    match output {
        Err(why) => panic!(why.to_string()),
        Ok(v) => {
            if matches.is_present("status") {
                let reader = ReaderBuilder::new().ascii().from_reader(v.as_bytes());
                match print_status(reader) {
                    Err(v) => panic!(v.to_string()),
                    _ => (),
                }
                /*
                 */
            } else {
                println!("{}", v);
            }
        }
    }
}
