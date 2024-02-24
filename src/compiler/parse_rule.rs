use std::usize;

use super::Compiler;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            _ => Precedence::Primary,
        }
    }
}

impl Into<u8> for Precedence {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct ParseRule {
    pub prefix_handler: Option<fn(&mut Compiler)>,
    pub infix_handler: Option<fn(&mut Compiler)>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(
        prefix_handler: Option<fn(&mut Compiler)>,
        infix_handler: Option<fn(&mut Compiler)>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix_handler,
            infix_handler,
            precedence,
        }
    }
}
