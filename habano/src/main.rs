use std::time::Instant;

mod error;
use error::{FileErrorPackage, HabanoError, LexerErrorPackage};

#[derive(Debug)]
enum BfCommands {
    Add,
    Show,
}

fn parse_commands(contents: &str) -> Result<Vec<BfCommands>, HabanoError> {
    let mut commands = Vec::new();

    let mut chars = contents.chars().peekable();
    let mut line = 1;
    let mut position = 0;
    while let Some(c) = chars.next() {
        position += 1;
        match c {
            '+' => commands.push(BfCommands::Add),
            '.' => commands.push(BfCommands::Show),
            ' ' | '\t' => (),
            '\n' => {
                line += 1;
                position = 0;
                // Check for \r and skip it
                if let Some('\r') = chars.peek() {
                    chars.next();
                }
            }
            '\r' => {
                line += 1;
                position = 0;
                // Check for \n and skip it
                if let Some('\n') = chars.peek() {
                    chars.next();
                }
            }
            _ => {
                return Err(HabanoError::InvalidCharacter(LexerErrorPackage {
                    character: c,
                    position,
                    line,
                }))
            }
        }
    }

    Ok(commands)
}

fn execute() -> Result<(), HabanoError> {
    if std::env::args().len() != 2 {
        return Err(HabanoError::ArgumentError);
    }

    let filename = std::env::args().nth(1).unwrap();
    println!("Reading file: {}", filename);

    let contents = std::fs::read_to_string(filename.clone())
        .map_err(|_| HabanoError::CannotOpenFile(FileErrorPackage { filename }))?;

    let commands = parse_commands(&contents)?;
    for command in commands {
        println!("{:?}", command);
    }

    Ok(())
}

fn main() {
    let start = Instant::now();

    let result = match execute() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    result
}
