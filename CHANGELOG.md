# Changelog

## [Unreleased]

## [0.2.0] - 2019-12-01

### Added

* Added `install` command to install an OpenVPN (`.ovpn`) or Tunnelblick (`.tblk`) configuration from the command line.
* `connect` and `disconnect` accept `-a`/`--all` flags to connect or disconnect from all tunnels, respectively.
* Bundled Bash completion function with the application. Run `complete` to print the completion to standard output.

### Changed

* `status` now outputs traffic sent/received as human readable values. Use `--bytes` to display traffic as bytes.
* Ported AppleScript layer to JavaScript for Automation (JXA). `tunnelblickctl` now only supports OS X (macOS) Yosemite (10.10) and higher.

## [0.1.0] - 2016-09-05

Initial release

[Unreleased]: https://github.com/benwebber/tunnelblickctl/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/benwebber/tunnelblickctl/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/benwebber/tunnelblickctl/releases/tag/v0.1.0
