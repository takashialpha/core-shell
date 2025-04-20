use crate::run::{execute, parse};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::process::exit;

pub fn shell_loop() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("$ ") {
            Ok(line) => {
                let _ = rl.add_history_entry(&line); // Add the line to history (no file)
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
    Ok(())
}
