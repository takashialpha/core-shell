use crate::coresh_file::{CoreShellData, CoreShellFile};
use crate::env::{get_current_dir, get_prompt_symbol};
use crate::formatting::{Color, Formatter, Style};
use crate::run::{execute, parse};
use atty::Stream;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::process::exit;

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
                '$' => {
                    symbol_str =
                        Formatter::apply_formatting(symbol_str, Color::Green, Some(Style::Bold))
                }
                '#' => {
                    symbol_str =
                        Formatter::apply_formatting(symbol_str, Color::Red, Some(Style::Bold))
                }
                _ => {}
            }
        }

        format!("\n{}\n{}", dir_str, symbol_str)
    }
}

pub struct Shell {
    data: CoreShellData,
    rl: DefaultEditor,
}

impl Shell {
    pub fn init() {
        let rl = match DefaultEditor::new() {
            Ok(editor) => editor,
            Err(err) => {
                println!("Error: {:?}", err);
                return;
            }
        };

        let mut shell = Shell {
            data: CoreShellFile::load(),
            rl,
        };

        shell.load_history();
        shell.run_loop();
    }

    fn run_loop(&mut self) {
        loop {
            let prompt = Prompt::generate();
            match self.rl.readline(&prompt.prompt) {
                Ok(line) => {
                    self.data.history.lines.push(line.clone());
                    let _ = self.rl.add_history_entry(&line);
                    
                    if self.data.config.save_history {
                        CoreShellFile::save(&self.data);
                    }

                    if let Some((cmd, args)) = parse(line) {
                        execute(&cmd, &args);
                    }
                }

                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    exit(1);
                }
            }
        }
    }

    fn load_history(&mut self) {
        for entry in &self.data.history.lines {
            let _ = self.rl.add_history_entry(entry);
        }
    }
}
