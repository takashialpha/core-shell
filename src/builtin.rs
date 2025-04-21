use lazy_static::lazy_static;
use std::collections::HashMap;
use std::process;

use crate::env::builtins::{cd, env, export, pwd, unset, printenv};

use crate::formatting::builtins::echo;

pub type BuiltInCmd = fn(&[&str]);

fn cd_link(args: &[&str]) {
    cd(args);
}

fn exit(_: &[&str]) {
    process::exit(0);
}

fn echo_link(args: &[&str]) {
    echo(args);
}

fn pwd_link(_: &[&str]) {
    pwd();
}

fn env_link(args: &[&str]) {
    env(args);
}

fn export_link(args: &[&str]) {
    export(args);
}

fn unset_link(args: &[&str]) {
    unset(args);
}

fn printenv_link(args: &[&str]) {
    printenv(args);
}

//  fn false(_: &[&str]) {
//  
//  }

//  fn true(_: &[&str]) {
//      
//  }

lazy_static! {
    pub static ref BUILTINS: HashMap<&'static str, BuiltInCmd> = {
        let mut m = HashMap::new();
        m.insert("cd", cd_link as BuiltInCmd);
        m.insert("exit", exit as BuiltInCmd);
        m.insert("echo", echo_link as BuiltInCmd);
        m.insert("pwd", pwd_link as BuiltInCmd);
        m.insert("env", env_link as BuiltInCmd);
        m.insert("set", export_link as BuiltInCmd);
        m.insert("unset", unset_link as BuiltInCmd);
        m.insert("export", export_link as BuiltInCmd);
        m.insert("printenv", printenv_link as BuiltInCmd);
        //  m.insert("false", false as BuiltInCmd);
        //  m.insert("true", true as BuiltInCmd);
        m
    };
}
