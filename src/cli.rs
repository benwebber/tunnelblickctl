use clap::{App, AppSettings, Arg, SubCommand};

pub fn cli() -> App<'static, 'static> {
    return App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("connect")
            .about("Connect to a VPN")
            .arg(Arg::with_name("name")
                .help("VPN to connect to")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("disconnect")
            .about("Disconnect from a VPN")
            .arg(Arg::with_name("name")
                .help("VPN to disconnect from")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("list")
            .visible_aliases(&["ls"])
            .about("List VPN configurations"))
        .subcommand(SubCommand::with_name("launch").about("Launch Tunnelblick"))
        .subcommand(SubCommand::with_name("status").about("Show VPN connection status"))
        .subcommand(SubCommand::with_name("quit").about("Quit Tunnelblick"))
        .subcommand(SubCommand::with_name("version").about("Show version information"))
        .subcommand(SubCommand::with_name("complete").about("Print Bash completion").setting(AppSettings::Hidden));
}
