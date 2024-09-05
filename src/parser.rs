use crate::expression::Expression;
use crate::lexer::{Token, Lexer, LexerError};

use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    Lexer (LexerError),

    ExpectedToken (Token),
    UnexpectedToken(Token),

    ExpectedEndOfInput,
    UnexpectedEndOfInput,
}

pub type ParserResult = Result<Expression, ParserError>;

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            tokens: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        let expression = self.parse_or()?;

        match self.tokens.next() {
            None => Ok(expression),
            Some (Ok(token)) => Err(ParserError::UnexpectedToken(token)),
            Some (Err(error)) => Err(ParserError::Lexer(error)),
        }
    }

    fn parse_or(&mut self) -> ParserResult {
        let mut left = self.parse_and()?;

        loop {
            match self.tokens.peek() {
                Some(Ok(Token::Or)) => {
                    self.tokens.next();

                    let right = self.parse_and()?;

                    left = Expression::Or(Box::new(left), Box::new(right));
                }

                _ => break Ok(left),
            }
        }        
    }

    fn parse_and(&mut self) -> ParserResult {
        let mut left = self.parse_xor()?;

        loop {
            match self.tokens.peek() {
                Some(Ok(Token::And)) => {
                    self.tokens.next();

                    let right = self.parse_xor()?;

                    left = Expression::And(Box::new(left), Box::new(right));
                }
                _ => break Ok(left),
            }
        }
    }

    
    fn parse_xor(&mut self) -> ParserResult {
        let mut left = self.parse_atom()?;

        loop {
            match self.tokens.peek() {
                Some(Ok(Token::Xor)) => {
                    self.tokens.next();

                    let right = self.parse_atom()?;

                    left = Expression::Xor(Box::new(left), Box::new(right));
                }
                _ => break Ok(left),
            }
        }
    }

    fn parse_atom(&mut self) -> ParserResult {
        match self.tokens.next() {
            Some(Ok(Token::Var(name))) => Ok(Expression::Var(name)),

            Some(Ok(Token::Not)) => {
                let expr = self.parse_atom()?;

                Ok(Expression::Not(Box::new(expr)))
            }

            Some(Ok(Token::LParen)) => {
                let expr = self.parse_or()?;

                match self.tokens.next() {
                    Some(Ok(Token::RParen)) => Ok(expr),
                    _ => Err(ParserError::ExpectedToken(Token::RParen)),
                }
            }

            Some(Ok(token)) => Err(ParserError::UnexpectedToken(token)),
            Some(Err(error)) => Err(ParserError::Lexer(error)),

            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_var() {
        let lexer = Lexer::new("a");
        let mut parser = Parser::new(lexer);

        assert_eq!(parser.parse(), Ok(Expression::Var("a".to_string())));
    }

    #[test]
    fn parse_and_expression() {
        let lexer = Lexer::new("a&b");
        let mut parser = Parser::new(lexer);

        assert_eq!(
            parser.parse(),

            Ok(
                Expression::And(
                    Box::new(Expression::Var("a".to_string())),
                    Box::new(Expression::Var("b".to_string())),
                )
        )
        );
    }

    #[test]
    fn parse_or_expression() {
        let lexer = Lexer::new("a|b");
        let mut parser = Parser::new(lexer);

        assert_eq!(
            parser.parse(),

            Ok(
                Expression::Or(
                    Box::new(Expression::Var("a".to_string())),
                    Box::new(Expression::Var("b".to_string())),
                )
            )
        );
    }

    #[test]
    fn parse_complex_expression() {
        let lexer = Lexer::new("(a&b)|¬c");
        let mut parser = Parser::new(lexer);

        assert_eq!(
            parser.parse(),

            Ok(
                Expression::Or(
                    Box::new(
                        Expression::And(
                            Box::new(Expression::Var("a".to_string())),
                            Box::new(Expression::Var("b".to_string()))
                        )
                    ),
                    Box::new(
                        Expression::Not(
                            Box::new(Expression::Var("c".to_string()))
                        )
                    )
                )
            )
        );
    }
}