mod run;
mod shell;

use shell::shell_loop;

// implement clap to parse command line arguments here in the future

fn main() {
    // Avoids exit by SIGINT
    ctrlc::set_handler(|| {}).expect("Error: failed to set ctrl-c handler");

    shell_loop().unwrap();
}
