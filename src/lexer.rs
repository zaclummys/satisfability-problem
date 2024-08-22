#[derive(Debug, PartialEq, Eq)]
enum Token {
    Var(char),
    And,
    Or,
    Not,
    LeftParen,
    RightParen,
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    
    for ch in expression.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' => tokens.push(Token::Var(ch)),
            '∧' => tokens.push(Token::And),
            '∨' => tokens.push(Token::Or),
            '¬' => tokens.push(Token::Not),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ch => {
                panic!("Unexpected char: {}", ch);
            }
        }
    }

    tokens
}
