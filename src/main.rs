mod builtin;
mod env;
mod run;
mod shell;

use crate::shell::Shell;
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
        .about("A fast and light shell written in Rust")
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
        .get_matches();
    

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

