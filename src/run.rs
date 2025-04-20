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
    match Command::new(cmd).args(cmd_args).output() {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(_) => {}
    }
}

