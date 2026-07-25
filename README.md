# PortKeep

PortKeep is a tool to keep track of your local ports. It can be used as a CLI or run as a web server.

## Getting Started

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

### CLI

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

### Server

You can run the server with the following command:

```bash
$ portkeep serve
2026-07-24T21:37:53.3311672-03:00 [INFO] 
===================================================
------------------ PortKeep v0.1.0 ------------------
===================================================

2026-07-24T21:37:53.3342596-03:00 [INFO] 
    listening on http://0.0.0.0:7678
    listening on http://localhost:7678
```

Once the server is running, open your browser and go to <http://localhost:7678>, or whatever it says in the logs. You can change the host and port with the CLI flags:

```bash
$ portkeep serve -h
Serve portkeep

Usage: portkeep serve [OPTIONS]

Options:
  -H, --host <HOST>  Host to listen on [env: PORTKEEP_HOST=] [default: 0.0.0.0]
  -p, --port <PORT>  Port to listen on [env: PORTKEEP_PORT=] [default: 7678]
      --debug        Enable debug logging [env: PORTKEEP_DEBUG=]
  -h, --help         Print help
  -V, --version      Print version
```

There are also environment variables, see [Environment Variables](#environment-variables).

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
| `PORTKEEP_HOST` | `0.0.0.0` | Host to listen on |
| `PORTKEEP_PORT` | `7678` | Port to listen on |

## Audits

| Auditor | Audit Date | Version | Vulnerabilities |
| --- | --- | --- | --- |
| [cargo-audit](https://crates.io/crates/cargo-audit) | 2026-07-24 | 0.1.0 | 0 |

## License

This project is licensed under the [Mozilla Public License, version 2.0](https://www.mozilla.org/MPL/2.0/). See the [LICENSE](LICENSE) file for details.
