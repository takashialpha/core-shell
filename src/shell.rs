use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::run::{parse, execute};

pub fn shell_loop() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("$ ") {
            Ok(line) => {
                if let Some((cmd, args)) = parse(line) {
                    execute(&cmd, &args);
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    Ok(())
}

