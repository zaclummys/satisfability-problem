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