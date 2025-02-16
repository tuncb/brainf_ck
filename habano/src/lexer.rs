#[derive(Debug)]
pub enum BfTokenType {
    Add,
    Dec,
    IncrementPointer,
    DecrementPointer,
    LoopStart,
    LoopEnd,
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

pub struct Tokens {
    pub token_types: Vec<BfTokenType>,
    pub positions: Vec<TokenPosition>,
}

pub fn lex(contents: &str) -> Tokens {
    let mut token_types = Vec::new();
    let mut positions = Vec::new();

    let mut add_token = |token_type: BfTokenType, line: usize, column: usize| {
        token_types.push(token_type);
        positions.push(TokenPosition { line, column });
    };

    let mut line = 1;
    let mut column = 1;

    let mut chars = contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => add_token(BfTokenType::Add, line, column),
            '-' => add_token(BfTokenType::Dec, line, column),
            '>' => add_token(BfTokenType::IncrementPointer, line, column),
            '<' => add_token(BfTokenType::DecrementPointer, line, column),
            '[' => add_token(BfTokenType::LoopStart, line, column),
            ']' => add_token(BfTokenType::LoopEnd, line, column),
            ',' => add_token(BfTokenType::Input, line, column),
            '.' => add_token(BfTokenType::Output, line, column),
            '\n' => {
                // Check for \r that follows
                if chars.peek() == Some(&'\r') {
                    chars.next();
                }
                line += 1;
                column = 0;
            }
            '\r' => {
                // Check for \n that follows
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                line += 1;
                column = 0;
            }
            _ => (),
        }
        column += 1;
    }

    Tokens {
        token_types,
        positions,
    }
}
