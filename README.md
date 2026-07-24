# PortKeep

PortKeep is a tool to keep track of your local ports.

## Getting Started

PortKeep is a command line tool that can be used to add, list, and remove ports from a configuration file. It allows keeping track of all open ports.

### From Source

1. **Prerequisite:** Ensure you have the [Rust toolchain](https://rustup.rs/) installed (see [MSRV](#msrv) to know what Rust version you need).
2. Clone the repo:

    ```bash
    git clone https://github.com/DarkCeptor44/portkeep.git
    cd portkeep
    ```

3. (Optional) Run in development mode (not recommended):

    ```bash
    cargo run
    ```

    Running other commands requires starting with `cargo run` in the repository.

4. (Optional) Install it with release mode:

    ```bash
    cargo install --path .
    ```

    This allows you to use the `portkeep` command from anywhere.

5. Check the [Usage](#usage) section to know how to use the tool.

## Usage

```bash
$ portkeep -h
A lightweight CLI and web server to track and manage local port allocations.

Usage: portkeep <COMMAND>

Commands:
  add     Add a port
  list    List ports
  remove  Remove a port
  serve   Serve portkeep
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Once the command is installed you can add, list, and remove ports with the following commands:

```bash
# To add a port
portkeep add                   # Asks for a port number, description and confirmation before adding
portkeep add 22                # Asks for a description and confirmation before adding
portkeep add 22 SSH            # Asks for confirmation before adding
portkeep add 22 SSH --confirm  # Adds the port without asking for confirmation

# To list ports
portkeep list                  # Lists all the ports in the configuration file

# To remove a port
portkeep remove               # Asks for a port number and confirmation before removing
portkeep remove 22            # Asks for confirmation before removing
portkeep remove 22 --confirm  # Removes the port without asking for confirmation
```

## MSRV

The minimum supported Rust version is:

| Version | Edition | MSRV |
| --- | --- | --- |
| 0.1.0 | 2024 | 1.85.0 |

## Environment Variables

The following environment variables are currently supported, they are used if the CLI flags are not set:

| Variable | Default | Description |
| --- | --- | --- |
| `PORTKEEP_DEBUG` | `false` | Enable debug logging |

## Audits

| Auditor | Audit Date | Version | Vulnerabilities |
| --- | --- | --- | --- |
| [cargo-audit](https://crates.io/crates/cargo-audit) | 2026-07-24 | 0.1.0 | 0 |

## License

This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/MPL/2.0/). See the [LICENSE](LICENSE) file for details.
