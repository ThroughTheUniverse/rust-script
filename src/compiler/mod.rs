use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::{
    chunk::{opcode::OpCode, Chunk},
    scanner::{
        token::{Token, TokenKind},
        Scanner,
    },
    value::Value,
};

use self::{
    locals::Local,
    parse_rule::{Precedence, Rules},
    parser::Parser,
};

mod locals;
mod parse_and;
mod parse_binary;
mod parse_block;
mod parse_declaration;
mod parse_expression;
mod parse_expression_statement;
mod parse_for;
mod parse_grouping;
mod parse_identifier;
mod parse_if;
mod parse_let;
mod parse_literal;
mod parse_number;
mod parse_or;
mod parse_print;
mod parse_rule;
mod parse_statement;
mod parse_string;
mod parse_unary;
mod parse_while;
mod parser;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct Compiler<'a> {
    parser: Rc<RefCell<Parser>>,
    scanner: Rc<RefCell<Scanner>>,
    rules: Rc<Rules>,
    chunk: &'a mut Chunk,
    pub locals: Vec<Local>,
    pub scope_depth: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            parser: Rc::new(RefCell::new(Parser::new())),
            scanner: Rc::new(RefCell::new(Scanner::new(""))),
            rules: Rc::new(Rules::new()),
            chunk,
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    fn scanner(&self) -> RefMut<'_, Scanner> {
        self.scanner.borrow_mut()
    }

    fn parser(&self) -> RefMut<'_, Parser> {
        self.parser.borrow_mut()
    }

    fn rules(&self) -> &Rc<Rules> {
        &self.rules
    }

    pub fn compile(&mut self, source: &str) -> Result<(), InterpretError> {
        *self.scanner() = Scanner::new(source);
        self.advance();
        while !self.matches(TokenKind::EOF) {
            self.parse_declaration();
        }
        self.end_complier();
        if self.parser().had_error.get() {
            Err(InterpretError::CompileError)
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) {
        let current = self.parser().current.clone();
        self.parser().previous = current;
        loop {
            self.parser().current = self.scanner().scan_token();
            if self.parser().current.kind != TokenKind::Error {
                break;
            }
            let message = &self.parser().current.lexeme;
            self.parser().error_at_current(message);
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.parser().current.kind == kind {
            self.advance();
            return;
        }
        self.parser().error_at_current(message);
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.parser().current.kind == kind
    }

    fn emit_one_byte<T: Into<u8>>(&mut self, byte: T) {
        let line_number = self.parser().previous.line_number;
        self.chunk.push_bytecode(byte.into(), line_number);
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

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while self.locals.len() > 0
            && self
                .locals
                .last()
                .unwrap()
                .depth
                .is_some_and(|depth| depth > self.scope_depth)
        {
            self.emit_one_byte(OpCode::Pop);
            self.locals.pop();
        }
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

    fn emit_identifier_constant(&mut self, name: String) -> u8 {
        self.make_constant(Value::String(name))
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let kind = self.parser().previous.kind;
        if let Some(prefix_handler) = self.rules().get(kind).prefix_handler {
            let can_assign = precedence <= Precedence::Assignment;
            prefix_handler(self, can_assign);
            while precedence <= self.rules().get(self.parser().current.kind).precedence {
                self.advance();
                let kind = self.parser().previous.kind;
                if let Some(infix_handler) = self.rules().get(kind).infix_handler {
                    infix_handler(self, can_assign);
                }
                if can_assign && self.matches(TokenKind::Equal) {
                    self.parser().error("Invalid assignment target.");
                }
            }
        } else {
            self.parser().error("Expect expression");
        }
    }

    fn synchronize(&mut self) {
        use TokenKind::*;
        self.parser().is_panic_mode.set(false);

        while self.parser().current.kind != EOF {
            if self.parser().previous.kind == Semicolon {
                return;
            }

            let kind = self.parser().current.kind;

            match kind {
                Struct | Fn | Let | For | If | While | Print | Return => return,
                _ => self.advance(),
            }
        }
    }

    fn add_local(&mut self, name: Token) {
        if self.locals.len() == u8::MAX as usize {
            self.parser().error("Too many local variables in function.");
            return;
        }
        self.locals.push(Local::new(name, None));
    }
}
