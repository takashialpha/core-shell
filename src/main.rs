mod builtin;
mod env;
mod run;
mod shell;

use crate::shell::Shell;
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
        .get_matches();

    if matches.get_flag("license") {
        println!("Core Shell is licensed under the Apache-2.0");
        return;
    }

    Shell::init();
}

