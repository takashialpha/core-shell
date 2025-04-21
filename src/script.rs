use crate::run::{execute, parse};
use std::path::Path;

pub fn run_script(path: &Path) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: could not read script file");
            return;
        }
    };

    for line in content.lines() {
        if let Some((cmd, args)) = parse(line.to_string()) {
            execute(&cmd, &args);
        }
    }
}
