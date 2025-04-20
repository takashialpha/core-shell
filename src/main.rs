mod run;
mod shell;

use shell::shell_loop;

fn main() {
    shell_loop().unwrap();
}

