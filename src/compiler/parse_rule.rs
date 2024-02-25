use std::{collections::HashMap, usize};

use crate::scanner::token::TokenKind;

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

pub struct Rules(pub HashMap<TokenKind, ParseRule>);

impl Rules {
    pub fn new() -> Self {
        Self(HashMap::from([
            (
                TokenKind::LeftParen,
                ParseRule::new(Some(|c| c.parse_grouping()), None, Precedence::None),
            ),
            (
                TokenKind::RightParen,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::LeftBrace,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::RightBrace,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Comma,
                ParseRule::new(None, None, Precedence::None),
            ),
            (TokenKind::Dot, ParseRule::new(None, None, Precedence::Call)),
            (
                TokenKind::Minus,
                ParseRule::new(
                    Some(|c| c.parse_unary()),
                    Some(|c| c.parse_binary()),
                    Precedence::Term,
                ),
            ),
            (
                TokenKind::Plus,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Term),
            ),
            (
                TokenKind::Semicolon,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Slash,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Factor),
            ),
            (
                TokenKind::Star,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Factor),
            ),
            (
                TokenKind::Bang,
                ParseRule::new(Some(|c| c.parse_unary()), None, Precedence::None),
            ),
            (
                TokenKind::BangEqual,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Equality),
            ),
            (
                TokenKind::Equal,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::EqualEqual,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Equality),
            ),
            (
                TokenKind::Greater,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Comparison),
            ),
            (
                TokenKind::GreaterEqual,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Comparison),
            ),
            (
                TokenKind::Less,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Comparison),
            ),
            (
                TokenKind::LessEqual,
                ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Comparison),
            ),
            (
                TokenKind::Identifier,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::String,
                ParseRule::new(Some(|c| c.parser_string()), None, Precedence::None),
            ),
            (
                TokenKind::Number,
                ParseRule::new(Some(|c| c.parse_number()), None, Precedence::None),
            ),
            (TokenKind::And, ParseRule::new(None, None, Precedence::And)),
            (
                TokenKind::Struct,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Else,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::False,
                ParseRule::new(Some(|c| c.parser_literal()), None, Precedence::None),
            ),
            (TokenKind::For, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Fn, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::If, ParseRule::new(None, None, Precedence::None)),
            (
                TokenKind::None,
                ParseRule::new(Some(|c| c.parser_literal()), None, Precedence::None),
            ),
            (TokenKind::Or, ParseRule::new(None, None, Precedence::Or)),
            (
                TokenKind::Print,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Return,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Super,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Self_,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::True,
                ParseRule::new(Some(|c| c.parser_literal()), None, Precedence::None),
            ),
            (TokenKind::Let, ParseRule::new(None, None, Precedence::None)),
            (
                TokenKind::While,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Error,
                ParseRule::new(None, None, Precedence::None),
            ),
            (TokenKind::EOF, ParseRule::new(None, None, Precedence::None)),
        ]))
    }

    pub fn get(&self, kind: TokenKind) -> &ParseRule {
        self.0.get(&kind).unwrap()
    }
}
