use crate::compiler::Compiler;
use crate::scanner::token::TokenKind;
use std::collections::HashMap;

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
    pub prefix_handler: Option<fn(&mut Compiler, can_assign: bool)>,
    pub infix_handler: Option<fn(&mut Compiler, can_assign: bool)>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(
        prefix_handler: Option<fn(&mut Compiler, can_assign: bool)>,
        infix_handler: Option<fn(&mut Compiler, can_assign: bool)>,
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
                ParseRule::new(
                    Some(|c, can_assign| c.parse_grouping_expression(can_assign)),
                    Some(|c, can_assign| c.parse_fn_call_expression(can_assign)),
                    Precedence::Call,
                ),
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
            (
                TokenKind::Dot,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_dot_expression(can_assign)),
                    Precedence::Call,
                ),
            ),
            (
                TokenKind::Minus,
                ParseRule::new(
                    Some(|c, can_assign| c.parse_unary_expression(can_assign)),
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Term,
                ),
            ),
            (
                TokenKind::Plus,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Term,
                ),
            ),
            (
                TokenKind::Modulo,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Factor,
                ),
            ),
            (
                TokenKind::Power,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Factor,
                ),
            ),
            (
                TokenKind::Semicolon,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Slash,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Factor,
                ),
            ),
            (
                TokenKind::Star,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Factor,
                ),
            ),
            (
                TokenKind::Bang,
                ParseRule::new(
                    Some(|c, can_assign| c.parse_unary_expression(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::BangEqual,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Equality,
                ),
            ),
            (
                TokenKind::Equal,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::EqualEqual,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Equality,
                ),
            ),
            (
                TokenKind::Greater,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Comparison,
                ),
            ),
            (
                TokenKind::GreaterEqual,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Comparison,
                ),
            ),
            (
                TokenKind::Less,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Comparison,
                ),
            ),
            (
                TokenKind::LessEqual,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_binary_expression(can_assign)),
                    Precedence::Comparison,
                ),
            ),
            (
                TokenKind::Identifier,
                ParseRule::new(
                    Some(|c, can_assign| c.parse_variable_expression(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::String,
                ParseRule::new(
                    Some(|c, can_assign| c.parser_string_literal(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::Number,
                ParseRule::new(
                    Some(|c, can_assign| c.parse_number_literal(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::And,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_and_expression(can_assign)),
                    Precedence::And,
                ),
            ),
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
                ParseRule::new(
                    Some(|c, can_assign| c.parser_literal(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (TokenKind::For, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::Fn, ParseRule::new(None, None, Precedence::None)),
            (TokenKind::If, ParseRule::new(None, None, Precedence::None)),
            (
                TokenKind::None,
                ParseRule::new(
                    Some(|c, can_assign| c.parser_literal(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::Or,
                ParseRule::new(
                    None,
                    Some(|c, can_assign| c.parse_or_expression(can_assign)),
                    Precedence::Or,
                ),
            ),
            (
                TokenKind::Print,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Return,
                ParseRule::new(None, None, Precedence::None),
            ),
            (
                TokenKind::Self_,
                ParseRule::new(
                    Some(|c, can_assign| c.parse_self(can_assign)),
                    None,
                    Precedence::None,
                ),
            ),
            (
                TokenKind::True,
                ParseRule::new(
                    Some(|c, can_assign| c.parser_literal(can_assign)),
                    None,
                    Precedence::None,
                ),
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
