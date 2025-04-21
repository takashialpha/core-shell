use std::collections::HashMap;
use std::process;

use lazy_static::lazy_static;

pub type BuiltInCmd = fn(&[&str]);

fn cd(args: &[&str]) {
    match args.len() {
        0 => {}
        1 => {
            let path = std::path::Path::new(args[0]);
            if let Err(e) = std::env::set_current_dir(path) {
                println!("cd: {}", e);
            }
        }
        _ => {
            println!("Too many arguments for a cd command.");
        }
    }
}

fn exit(_: &[&str]) {
    process::exit(0);
}

fn echo(args: &[&str]) {
    for arg in args {
        print!("{}", arg);
        if !arg.ends_with('\n') {
            print!(" ");
        }
    }
    println!();
}

lazy_static! {
    pub static ref BUILTINS: HashMap<&'static str, BuiltInCmd> = {
        let mut m = HashMap::new();
        m.insert("cd", cd as BuiltInCmd);
        m.insert("exit", exit as BuiltInCmd);
        m.insert("echo", echo as BuiltInCmd);
        m
    };
}
