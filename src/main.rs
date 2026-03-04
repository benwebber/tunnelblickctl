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
                writer.write_record(&["NAME", "STATE", "AUTOCONNECT", "TX", "RX"])?;
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
