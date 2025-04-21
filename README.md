
# core-shell

**A fast, POSIX-like shell written in Rust.**


## Features

- Interactive shell with line editing
- Builtin command support
- `-c` for one-shot commands
- Script execution from file
- Flags: `--version`, `--license`, `--help`
- Clean and fast parser
- Meant to be lightweight enough for `/bin/sh`


## Usage

```sh
core-shell [OPTIONS] [SCRIPT]
```

### Arguments:
- `[SCRIPT]` — Path to a shell script to execute

### Options:
- `-c, --command <command>` — Run a single command and exit (e.g. `-c "echo hello"`)
- `--license` — Displays license information
- `-h, --help` — Print help
- `-V, --version` — Print version


## Installation

### Build (release mode)

```sh
git clone https://github.com/yourusername/core-shell
cd core-shell
cargo build --release
```

### Install to `/bin` or as `/bin/sh`

```sh
sudo cp target/release/core-shell /bin/core-shell
sudo ln -sf /bin/core-shell /bin/sh - this part sets the core-shell as your sh
```

> ⚠️ Warning: replacing `/bin/sh` can be dangerous if you dont know what you're doing, make sure to test the shell before using it as your sh.
> Bugs can be found as the project is in constant development.

## Statistics
Lines of rust code: 499 (measured by [cloc](github.com/AlDanial/cloc))
Binary size: 1.2 MB (almost a megabyte)
CPU usage: 0.0% (in modern computers)
Memory usage: 2.9 MB (RAM usage)
> ⚠️ Warning: The statistics may vary depending on the system.

## Dependencies

- [clap](https://docs.rs/clap) – command-line argument parser
- [rustyline](https://docs.rs/rustyline) – readline implementation
- [shell-words](https://docs.rs/shell-words) – simple shell-like argument parser
... and it continues
check it out in our [Cargo.toml](https://github.com/takashialpha/core-shell/blob/main/Cargo.toml)

observations: you dont need to install these dependencies, they are automatically installed by cargo when you build the project.

## License

Apache-2.0

## Wiki

**Please read our [wiki](https://github.com/takashialpha/core-shell/wiki) for more information.**

## Credits

Huge thanks to the maintainers of the dependencies and the Rust ecosystem.

> ⚠️ Warning: Some information may be outdated as this README is updated only with new releases.

***

core-shell readme 0.3.0 from core-shell 0.3.0
