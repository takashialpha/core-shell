use crate::env::{get_current_dir, get_prompt_symbol};
use crate::run::{execute, parse};
use crate::formatting::{Color, Style, Formatter};
use std::process::exit;
use atty::Stream;

struct Prompt {
    prompt: String,
}

impl Prompt {
    fn generate() -> Self {
        let dir = get_current_dir().unwrap_or("".to_string());
        let symbol = get_prompt_symbol();
        let prompt = Self::format_prompt(dir, symbol);
        Prompt { prompt }
    }

    fn format_prompt(dir: String, symbol: char) -> String {
        let colored = atty::is(Stream::Stdout);

        let mut dir_str = format!("{}", dir);
        if colored {
            dir_str = Formatter::apply_formatting(dir_str, Color::Blue, None);
        }

        let mut symbol_str = format!("{} ", symbol);
        if colored {
            match symbol {
                '$' => symbol_str = Formatter::apply_formatting(symbol_str, Color::Green, Some(Style::Bold)),
                '#' => symbol_str = Formatter::apply_formatting(symbol_str, Color::Red, Some(Style::Bold)),
                _ => {}
            }
        }

        format!("\n{}\n{}", dir_str, symbol_str)
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
