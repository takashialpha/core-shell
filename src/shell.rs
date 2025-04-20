use crate::env::{get_current_dir, get_prompt_symbol};
use crate::run::{execute, parse};
use std::process::exit;

struct Prompt {
    prompt: String,
}

impl Prompt {
    fn generate() -> Self {
        Prompt {
            prompt: format!(
                "\n({})\n{} ",
                get_current_dir().unwrap_or("".to_string()),
                get_prompt_symbol()
            ),
        }
    }
}

pub struct Shell;

impl Shell {
    pub fn init() {
        Self::run_loop();
    }

    fn run_loop() {
        let mut rl = match rustyline::DefaultEditor::new() {
            Ok(editor) => editor,
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        };

        loop {
            let prompt = Prompt::generate();
            match rl.readline(&prompt.prompt) {
                Ok(line) => {
                    let _ = rl.add_history_entry(&line);
                    if let Some((cmd, args)) = parse(line) {
                        execute(&cmd, &args);
                    }
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {}
                Err(rustyline::error::ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    exit(1);
                }
            }
        }
    }
}
