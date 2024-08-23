use std::iter::Peekable;

use crate::lexer::{
    Token,
    Lexer,
    LexerError,
    LexerResult,
};

use crate::expression::Expression;


struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    pub fn new (lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            tokens: lexer.peekable()
        }
    }
}