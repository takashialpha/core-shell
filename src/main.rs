mod builtin;
mod env;
mod run;
mod shell;
mod script;

use crate::shell::Shell;
use crate::script::run_script;
use crate::run::{parse, execute};
use clap::{Command, Arg};

const VERSION: &str = "0.1.0";

fn main() {
    // Avoids exit by SIGINT
    ctrlc::set_handler(|| {}).expect("Error: failed to set ctrl-c handler");

    // Clap CLI argument parsing
    let matches = Command::new("Core Shell")
        .version(VERSION)
        .author("takashialpha <takashialpha@protonmail.com>")
        .about("A fast, minimal POSIX-like shell written in Rust.")
        .arg(
            Arg::new("license")
                .long("license")
                .help("Displays the license information")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .help("Run a single command and exit, e.g. -c \"echo hello\"")
                .num_args(1)
        )
        .arg(
            Arg::new("script")
                .value_name("SCRIPT")
                .help("Path to a shell script to execute")
                .value_parser(clap::value_parser!(std::path::PathBuf))
        )

        .get_matches();
    
    // script flag behavior
    if let Some(path) = matches.get_one::<std::path::PathBuf>("script") {
        run_script(path);
        return;
    }

    // license flag behavior
    if matches.get_flag("license") {
        println!("Core Shell is licensed under the Apache-2.0");
        return;
    }
    
    // command flag behavior
    if let Some(cmd) = matches.get_one::<String>("command") {
        if let Some((name, args)) = parse(cmd.to_string()) {
            execute(&name, &args);
        }
        return;
    }

    Shell::init();
}

