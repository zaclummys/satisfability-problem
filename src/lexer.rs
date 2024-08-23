use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Var(char),
    And,
    Or,
    Not,
    LParen,
    RParen,
    End,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedCharacter (char)
}

pub type LexerResult = Result<Token, LexerError>;

pub struct Lexer<'a> {
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {
    pub fn new (string: &'a str) -> Lexer<'a> {
        Lexer {
            chars: string.chars()
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
            .map(|ch| {
                match ch {
                    '∧' => Ok(Token::And),
                    '∨' => Ok(Token::Or),
                    '¬' => Ok(Token::Not),
                    '(' => Ok(Token::LParen),
                    ')' => Ok(Token::RParen),
                    'a'..='z' | 'A'..='Z' => Ok(Token::Var(ch)),
                    ch => Err(LexerError::UnexpectedCharacter(ch))
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tokenize_var() {
        let mut lexer = Lexer::new("a");
        
        assert_eq!(lexer.next(), Some(Ok(Token::Var('a'))));
    }

    #[test]
    fn should_tokenize_and() {
        let mut lexer = Lexer::new("∧");
        
        assert_eq!(lexer.next(), Some(Ok(Token::And)));
    }

    #[test]
    fn should_tokenize_or() {
        let mut lexer = Lexer::new("∨");
        
        assert_eq!(lexer.next(), Some(Ok(Token::Or)));
    }

    #[test]
    fn should_tokenize_not() {
        let mut lexer = Lexer::new("¬");
        
        assert_eq!(lexer.next(), Some(Ok(Token::Not)));
    }

    #[test]
    fn should_tokenize_lparen() {
        let mut lexer = Lexer::new("(");
        
        assert_eq!(lexer.next(), Some(Ok(Token::LParen)));
    }

    #[test]
    fn should_tokenize_rparen() {
        let mut lexer = Lexer::new(")");
        
        assert_eq!(lexer.next(), Some(Ok(Token::RParen)));
    }

    #[test]
    fn should_tokenize_invalid_char() {
        let mut lexer = Lexer::new("!");
        
        assert_eq!(lexer.next(), Some(Err(LexerError::UnexpectedCharacter('!'))));
    }
}
