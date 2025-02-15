use std::time::Instant;

mod error;
use error::{FileErrorPackage, HabanoError};

#[derive(Debug)]
enum BfCommands {
    Add,
    Dec,
    IncrementPointer,
    DecrementPointer,
    Show,
    Ignore,
}

fn parse_commands(contents: &str) -> Vec<BfCommands> {
    let mut commands = Vec::new();

    for c in contents.chars() {
        match c {
            '+' => commands.push(BfCommands::Add),
            '.' => commands.push(BfCommands::Show),
            '-' => commands.push(BfCommands::Dec),
            '<' => commands.push(BfCommands::DecrementPointer),
            '>' => commands.push(BfCommands::IncrementPointer),
            _ => commands.push(BfCommands::Ignore),
        }
    }

    commands
}

fn run_commands(commands: Vec<BfCommands>) {
    let mut memory = [0i8; 30000];
    let mut pointer = 0;

    for command in commands {
        match command {
            BfCommands::Add => memory[pointer] = memory[pointer].wrapping_add(1),
            BfCommands::Dec => memory[pointer] = memory[pointer].wrapping_sub(1),
            BfCommands::IncrementPointer => pointer = pointer.wrapping_add(1),
            BfCommands::DecrementPointer => pointer = pointer.wrapping_sub(1),
            BfCommands::Show => {
                // Convert i8 to u8 using bitwise operations
                let byte = memory[pointer] as u8;
                print!("{}", byte as char);
            }
            BfCommands::Ignore => (),
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

    let commands = parse_commands(&contents);
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
