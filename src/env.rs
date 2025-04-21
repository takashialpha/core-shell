use std::env;
use std::path::Path;

pub fn get_home_dir() -> Option<String> {
    env::var("HOME").ok()
}

pub fn get_current_dir() -> Option<String> {
    let current = env::current_dir().ok()?.to_string_lossy().into_owned();
    let home_path = get_home_dir()?;
    Some(
        current
            .strip_prefix(&home_path)
            .map_or_else(|| current.clone(), |s| format!("~{}", s)),
    )
}

pub fn get_prompt_symbol() -> char {
    unsafe {
        if libc::geteuid() == 0 {
            return '#';
        }
    }
    '$'
}

pub mod builtins {
    use super::*;
    use std::process::Command;

    fn valid_ident(id: &str) -> bool {
        !id.is_empty() && id.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    pub fn cd(args: &[&str]) {
        match args.len() {
            0 => {}
            1 => {
                let path = Path::new(args[0]);
                if let Err(e) = env::set_current_dir(path) {
                    println!("cd: {}", e);
                }
            }
            _ => {
                println!("Too many arguments for a cd command.");
            }
        }
    }

    pub fn pwd() {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("pwd: {}", e),
        }
    }

    pub fn env(args: &[&str]) {
        let pos = args
            .iter()
            .position(|s| !s.contains('='))
            .unwrap_or(args.len());
        for kv in &args[..pos] {
            let mut parts = kv.splitn(2, '=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap_or("");
            if !valid_ident(key) {
                eprintln!("env: invalid argument: {}", kv);
                return;
            }
            env::set_var(key, value);
        }
        let rest = &args[pos..];
        if rest.is_empty() {
            env::vars().for_each(|(k, v)| println!("{}={}", k, v));
            return;
        }
        if let Err(e) = Command::new(rest[0]).args(&rest[1..]).status() {
            eprintln!("env: {}", e);
            std::process::exit(127);
        }
    }

    pub fn export(args: &[&str]) {
        if args.is_empty() {
            env::vars().for_each(|(k, v)| println!("declare -x {}='{}'", k, v));
            return;
        }
        for arg in args {
            let mut parts = arg.splitn(2, '=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap_or("");
            if !valid_ident(key) {
                eprintln!("export: not an identifier: {}", key);
                continue;
            }
            env::set_var(key, value);
        }
    }

    pub fn unset(args: &[&str]) {
        if args.is_empty() {
            eprintln!("unset: not enough arguments");
            return;
        }
        for key in args {
            if valid_ident(key) {
                env::remove_var(key);
                continue;
            }
            eprintln!("unset: invalid name: {}", key);
        }
    }

    pub fn printenv(args: &[&str]) {
        if args.is_empty() {
            env::vars().for_each(|(k, v)| println!("{}={}", k, v));
            return;
        }
        for key in args {
            if let Ok(val) = env::var(key) {
                println!("{}", val);
            }
        }
    }
}
