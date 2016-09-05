#[macro_use]
extern crate clap;
extern crate tabwriter;

mod cli;
mod tunnelblick;

fn version() -> String {
    let cli_version = crate_version!();
    let app_version = tunnelblick::Client::command("getVersion").send();
    return format!("{} {}\nTunnelblick {}",
                   env!("CARGO_PKG_NAME"),
                   cli_version,
                   app_version);
}

fn main() {
    let matches = cli::cli().get_matches();

    if matches.is_present("version") {
        print!("{}", version());
        return;
    }

    let output = match matches.subcommand() {
        ("connect", Some(m)) => {
            tunnelblick::Client::command("connect").arg(m.value_of("name").unwrap()).send()
        }
        ("disconnect", Some(m)) => {
            tunnelblick::Client::command("disconnect").arg(m.value_of("name").unwrap()).send()
        }
        ("list", Some(_)) => tunnelblick::Client::command("listTunnels").send(),
        ("status", Some(_)) => tunnelblick::Client::command("showStatus").send(),
        ("quit", Some(_)) => tunnelblick::Client::command("quit").send(),
        ("launch", Some(_)) => tunnelblick::Client::command("run").send(),
        _ => "".to_owned(),
    };
    print!("{}", output);
}
