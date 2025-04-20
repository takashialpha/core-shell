mod builtin;
mod env;
mod run;
mod shell;

use crate::shell::Shell;

// implement clap to parse command line arguments here in the future

fn main() {
    // Avoids exit by SIGINT
    ctrlc::set_handler(|| {}).expect("Error: failed to set ctrl-c handler");

    Shell::init();
}
