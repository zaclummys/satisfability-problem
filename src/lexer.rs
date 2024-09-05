use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Var(String),
    And,
    Or,
    Not,
    LParen,
    RParen,
    Xor,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedCharacter (char)
}

pub type LexerResult = Result<Token, LexerError>;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new (string: &'a str) -> Lexer<'a> {
        Lexer {
            chars: string.chars().peekable()
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some (ch) if ch.is_ascii_whitespace() => continue,

                Some (ch) => break match ch {
                    '^' => Some(Ok(Token::Xor)),
                    '|' => Some(Ok(Token::Or)),
                    '&' => Some(Ok(Token::And)),
                    '¬' => Some(Ok(Token::Not)),
                    '(' => Some(Ok(Token::LParen)),
                    ')' => Some(Ok(Token::RParen)),

                    ch if ch.is_ascii_alphanumeric() => {
                        let mut string = String::from(ch);

                        loop {
                            match self.chars.next_if(|ch| ch.is_ascii_alphanumeric()) {
                                Some (ch) => string.push(ch),
                                _ => break Some(Ok(Token::Var(string))),
                            }
                        }
                    }

                    _ => Some(Err(LexerError::UnexpectedCharacter(ch))),
                },

                None => break None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_tokenize_var() {
        let mut lexer = Lexer::new("abc");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Var("abc".to_string())))
        );
    }

    #[test]
    fn should_tokenize_and() {
        let mut lexer = Lexer::new("&");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::And))
        );
    }

    #[test]
    fn should_tokenize_or() {
        let mut lexer = Lexer::new("|");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Or))
        );
    }

    #[test]
    fn should_tokenize_not() {
        let mut lexer = Lexer::new("¬");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::Not))
        );
    }

    #[test]
    fn should_tokenize_lparen() {
        let mut lexer = Lexer::new("(");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::LParen))
        );
    }

    #[test]
    fn should_tokenize_rparen() {
        let mut lexer = Lexer::new(")");
        
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::RParen))
        );
    }

    #[test]
    fn should_tokenize_invalid_char() {
        let mut lexer = Lexer::new("!");
        
        assert_eq!(
            lexer.next(),
            Some(Err(LexerError::UnexpectedCharacter('!')))
        );
    }
}
