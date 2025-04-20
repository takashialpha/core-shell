mod run;
mod shell;

use shell::shell_loop;

// implement clap to parse command line arguments here in the future

fn main() {
    shell_loop().unwrap();
}

