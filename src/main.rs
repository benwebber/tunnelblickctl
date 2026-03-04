use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};
use humansize::{BINARY, format_size};
use tabwriter::TabWriter;

mod tunnelblick;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
        Self {
            autoconnect: config.autoconnect.clone(),
            state: config.state.clone(),
            name: config.name.clone(),
            bytes_in: format_size(config.bytes_in, BINARY),
            bytes_out: format_size(config.bytes_out, BINARY),
        }
    }
}

#[derive(Parser)]
#[command(
    name = "tunnelblickctl",
    disable_version_flag = true,
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(hide = true)]
    Complete,
    #[command(group(ArgGroup::new("tunnel").required(true)))]
    Connect {
        #[arg(group = "tunnel")]
        vpn: Option<String>,
        #[arg(short, long, group = "tunnel")]
        all: bool,
    },
    #[command(group(ArgGroup::new("tunnel").required(true)))]
    Disconnect {
        #[arg(group = "tunnel")]
        vpn: Option<String>,
        #[arg(short, long, group = "tunnel")]
        all: bool,
    },
    Install {
        file: PathBuf,
    },
    Launch,
    #[command(visible_alias = "ls")]
    List,
    Status {
        #[arg(long)]
        bytes: bool,
    },
    Quit,
    Version,
}

fn main() -> Result<(), Box<dyn Error>> {
    let version = option_env!("version").unwrap_or(env!("CARGO_PKG_VERSION"));
    let cli = Cli::parse();

    if let Commands::Complete = cli.command {
        print!("{}", include_str!("../contrib/tunnelblick.bash"));
        return Ok(());
    }

    let bytes = matches!(&cli.command, Commands::Status { bytes: true });

    let command = match cli.command {
        Commands::Connect {
            vpn: Some(name), ..
        } => tunnelblick::Command::Connect(name),
        Commands::Connect { .. } => tunnelblick::Command::ConnectAll,
        Commands::Disconnect {
            vpn: Some(name), ..
        } => tunnelblick::Command::Disconnect(name),
        Commands::Disconnect { .. } => tunnelblick::Command::DisconnectAll,
        Commands::Install { file } => {
            let abs = fs::canonicalize(&file)?;
            tunnelblick::Command::Install(abs.to_str().unwrap().to_string())
        }
        Commands::List => tunnelblick::Command::List,
        Commands::Status { .. } => tunnelblick::Command::GetStatus,
        Commands::Quit => tunnelblick::Command::Quit,
        Commands::Launch => tunnelblick::Command::Launch,
        Commands::Version => tunnelblick::Command::GetVersion,
        Commands::Complete => unreachable!(),
    };

    let response = command.execute();

    match (command, response) {
        (tunnelblick::Command::List, Ok(data)) => match data {
            tunnelblick::ResponseData::StringArray(configs) => {
                for config in configs.iter() {
                    println!("{}", config);
                }
            }
            _ => unreachable!(),
        },
        (tunnelblick::Command::GetStatus, Ok(data)) => match data {
            tunnelblick::ResponseData::Configurations(configs) => {
                let tab_writer = TabWriter::new(io::stdout());
                let mut writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b'\t')
                    .from_writer(tab_writer);
                writer.write_record(["NAME", "STATE", "AUTOCONNECT", "TX", "RX"])?;
                for config in configs.iter() {
                    if bytes {
                        writer.serialize(config)?;
                    } else {
                        let human_config = HumanConfiguration::from(config);
                        writer.serialize(human_config)?;
                    }
                }
            }
            _ => unreachable!(),
        },
        (tunnelblick::Command::GetVersion, Ok(data)) => {
            println!(
                "{} {}\nTunnelblick {}",
                env!("CARGO_PKG_NAME"),
                version,
                data
            );
        }
        (tunnelblick::Command::GetVersion, Err(_)) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), version);
        }
        (_, Err(message)) => {
            return Err(message);
        }
        _ => (),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands, HumanConfiguration, tunnelblick};
    use clap::Parser;

    macro_rules! test_cli {
        ($name:ident, [$($arg:expr),*], Err(_)) => {
            #[test]
            fn $name() {
                assert!(Cli::try_parse_from(["tunnelblickctl" $(, $arg)*]).is_err());
            }
        };
        ($name:ident, [$($arg:expr),*], $pat:pat) => {
            #[test]
            fn $name() {
                let result = Cli::try_parse_from(["tunnelblickctl" $(, $arg)*])
                    .map(|c| c.command);
                assert!(matches!(result, $pat));
            }
        };
    }

    test_cli!(
        connect_vpn,
        ["connect", "work-vpn"],
        Ok(Commands::Connect {
            vpn: Some(_),
            all: false
        })
    );
    test_cli!(
        connect_all,
        ["connect", "--all"],
        Ok(Commands::Connect {
            vpn: None,
            all: true
        })
    );
    test_cli!(connect_no_args, ["connect"], Err(_));
    test_cli!(connect_exclusive, ["connect", "work-vpn", "--all"], Err(_));
    test_cli!(
        disconnect_vpn,
        ["disconnect", "work-vpn"],
        Ok(Commands::Disconnect {
            vpn: Some(_),
            all: false
        })
    );
    test_cli!(
        disconnect_all,
        ["disconnect", "--all"],
        Ok(Commands::Disconnect {
            vpn: None,
            all: true
        })
    );
    test_cli!(list_alias, ["ls"], Ok(Commands::List));
    test_cli!(
        status_default,
        ["status"],
        Ok(Commands::Status { bytes: false })
    );
    test_cli!(
        status_bytes,
        ["status", "--bytes"],
        Ok(Commands::Status { bytes: true })
    );
    test_cli!(no_subcommand, [], Err(_));

    macro_rules! test_human_config {
        ($name:ident, $bytes_in:expr, $bytes_out:expr, $expected_in:expr, $expected_out:expr) => {
            #[test]
            fn $name() {
                let config = tunnelblick::Configuration {
                    name: String::new(),
                    state: String::new(),
                    autoconnect: String::new(),
                    bytes_in: $bytes_in,
                    bytes_out: $bytes_out,
                };
                let human = HumanConfiguration::from(&config);
                assert_eq!(human.bytes_in, $expected_in);
                assert_eq!(human.bytes_out, $expected_out);
            }
        };
    }

    test_human_config!(human_config_zero, 0, 0, "0 B", "0 B");
    test_human_config!(human_config_kib, 1024, 0, "1 KiB", "0 B");
    test_human_config!(human_config_mib, 0, 1_048_576, "0 B", "1 MiB");
}
