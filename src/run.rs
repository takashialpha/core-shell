use crate::builtin::BUILTINS;
use std::process::Command;

pub fn parse(line: String) -> Option<(String, Vec<String>)> {
    let input = line.trim();
    if input.is_empty() {
        return None;
    }

    match shell_words::split(input) {
        Ok(args) => {
            if let Some((cmd, cmd_args)) = args.split_first() {
                return Some((cmd.to_string(), cmd_args.to_vec()));
            }
        }
        Err(_) => {}
    }

    None
}

pub fn execute(cmd: &str, cmd_args: &[String]) {
    if catch_builtins(cmd, cmd_args) {
        return;
    }

    if let Err(e) = Command::new(cmd)
        .args(cmd_args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .and_then(|mut child| child.wait())
    {
        eprintln!("Error: {}", e);
    }
}

fn catch_builtins(cmd: &str, cmd_args: &[String]) -> bool {
    if let Some(builtin) = BUILTINS.get(cmd) {
        builtin(&cmd_args.iter().map(String::as_str).collect::<Vec<_>>());

        return true;
    }
    false
}
