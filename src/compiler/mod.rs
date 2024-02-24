use std::collections::HashMap;

use crate::{
    chunk::{opcode::OpCode, Chunk},
    scanner::{token::TokenKind, Scanner},
    value::Value,
};

use self::{
    parse_rule::{ParseRule, Precedence},
    parser::Parser,
};

mod parse_binary;
mod parse_expression;
mod parse_grouping;
mod parse_number;
mod parse_rule;
mod parse_unary;
mod parser;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct Compiler<'a> {
    pub parser: Parser,
    pub scanner: Scanner,
    chunk: &'a mut Chunk,
    rules: HashMap<TokenKind, ParseRule>,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            parser: Parser::new(),
            scanner: Scanner::new(""),
            chunk,
            rules: HashMap::from([
                (
                    TokenKind::LeftParen,
                    ParseRule::new(Some(|c| c.parse_grouping()), None, Precedence::None),
                ),
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
                    TokenKind::Slash,
                    ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Factor),
                ),
                (
                    TokenKind::Star,
                    ParseRule::new(None, Some(|c| c.parse_binary()), Precedence::Factor),
                ),
                (
                    TokenKind::Number,
                    ParseRule::new(Some(|c| c.parse_number()), None, Precedence::None),
                ),
                (TokenKind::EOF, ParseRule::new(None, None, Precedence::None)),
            ]),
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<(), InterpretError> {
        self.scanner = Scanner::new(source);
        self.advance();
        self.parse_expression();
        self.consume(TokenKind::EOF, "Expect end of expression");
        self.end_complier();
        if self.parser.had_error.get() {
            Err(InterpretError::CompileError)
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.kind != TokenKind::Error {
                break;
            }
            self.parser
                .error_at_current(self.parser.current.lexeme.as_str());
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.parser.current.kind == kind {
            self.advance();
            return;
        }
        self.parser.error_at_current(message);
    }

    fn emit_one_byte<T: Into<u8>>(&mut self, byte: T) {
        self.chunk
            .push_bytecode(byte.into(), self.parser.previous.line_number);
    }

    fn emit_two_bytes<T: Into<u8>, U: Into<u8>>(&mut self, byte1: T, byte2: U) {
        self.emit_one_byte(byte1.into());
        self.emit_one_byte(byte2.into());
    }

    fn emit_return(&mut self) {
        self.emit_one_byte(OpCode::Return);
    }

    fn end_complier(&mut self) {
        self.emit_return();
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        if let Some(index) = self.chunk.push_constant(value) {
            index
        } else {
            0
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit_two_bytes(OpCode::Constant, index);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(prefix_handler) = self
            .rules
            .get(&self.parser.previous.kind)
            .unwrap()
            .prefix_handler
        {
            prefix_handler(self);
            while precedence
                <= self
                    .rules
                    .get(&self.parser.current.kind)
                    .unwrap()
                    .precedence
            {
                self.advance();
                if let Some(infix_handler) = self
                    .rules
                    .get(&self.parser.previous.kind)
                    .unwrap()
                    .infix_handler
                {
                    infix_handler(self);
                }
            }
        } else {
            self.parser.error("Expect expression");
        }
    }
}
