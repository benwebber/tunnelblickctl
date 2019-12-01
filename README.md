# tunnelblickctl
[![Build status](https://ci.appveyor.com/api/projects/status/15lfq1l4svlf7t40/branch/master?svg=true)](https://ci.appveyor.com/project/benwebber/tunnelblickctl/branch/master)

Command-line interface for [Tunnelblick](https://tunnelblick.net/), the *de facto* OpenVPN client for OS X.

## Install

Download one of the [pre-compiled releases](https://github.com/benwebber/tunnelblickctl/releases/), then copy it to your `$PATH`.

## Usage

```
$ tunnelblickctl help
tunnelblickctl

USAGE:
    tunnelblickctl [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    connect       Connect to a VPN
    disconnect    Disconnect from a VPN
    help          Prints this message or the help of the given subcommand(s)
    install       Install an OpenVPN or Tunnelblick configuration
    launch        Launch Tunnelblick
    list          List VPN configurations [aliases: ls]
    quit          Quit Tunnelblick
    status        Show VPN connection status
    version       Show version information
```

## License

MIT
