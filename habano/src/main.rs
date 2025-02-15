use std::time::Instant;

mod error;
use error::{FileErrorPackage, HabanoError, LexerErrorPackage};

#[derive(Debug)]
enum BfCommands {
    Add,
    Dec,
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
            '-' => commands.push(BfCommands::Dec),
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

fn run_commands(commands: Vec<BfCommands>) {
    let mut memory = [0i8; 30000];
    let pointer = 0;

    for command in commands {
        match command {
            BfCommands::Add => memory[pointer] = memory[pointer].wrapping_add(1),
            BfCommands::Dec => memory[pointer] = memory[pointer].wrapping_sub(1),
            BfCommands::Show => {
                // Convert i8 to u8 using bitwise operations
                let byte = memory[pointer] as u8;
                print!("{}", byte as char);
            }
        }
    }
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
    run_commands(commands);

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
