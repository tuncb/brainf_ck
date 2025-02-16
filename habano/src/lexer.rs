#[derive(Debug)]
pub enum BfToken {
    Add,
    Dec,
    IncrementPointer,
    DecrementPointer,
    LoopStart,
    LoopEnd,
    Input,
    Output,
}

pub fn lex(contents: &str) -> Vec<BfToken> {
    let mut tokens = Vec::new();

    for c in contents.chars() {
        match c {
            '+' => tokens.push(BfToken::Add),
            '.' => tokens.push(BfToken::Output),
            ',' => tokens.push(BfToken::Input),
            '-' => tokens.push(BfToken::Dec),
            '<' => tokens.push(BfToken::DecrementPointer),
            '>' => tokens.push(BfToken::IncrementPointer),
            '[' => tokens.push(BfToken::LoopStart),
            ']' => tokens.push(BfToken::LoopEnd),
            _ => (),
        }
    }

    tokens
}
