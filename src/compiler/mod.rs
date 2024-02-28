use self::parser::{
    parse_rule::{Precedence, Rules},
    Parser,
};
use crate::{
    chunk::{opcode::OpCode, Chunk},
    object::function_object::FunctionObject,
    scanner::{
        token::{Token, TokenKind},
        Scanner,
    },
    value::Value,
};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

mod parse_declaration;
mod parse_expression;
mod parse_literal;
mod parse_statement;
mod parser;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum FunctionKind {
    Function,
    Method,
    Script,
    Initializer,
}

pub struct ClassCompiler {
    pub enclosing: RefCell<Option<Rc<ClassCompiler>>>,
    pub has_superclass: RefCell<bool>,
}

impl ClassCompiler {
    pub fn new() -> Self {
        Self {
            enclosing: RefCell::new(None),
            has_superclass: RefCell::new(false),
        }
    }
}

pub struct Local {
    pub name: Token,
    pub depth: Option<usize>,
}

impl Local {
    pub fn new(name: Token, depth: Option<usize>) -> Self {
        Self { name, depth }
    }
}

pub struct Compiler {
    parser: Rc<RefCell<Parser>>,
    scanner: Rc<RefCell<Scanner>>,
    function: FunctionObject,
    kind: FunctionKind,
    rules: Rc<Rules>,
    current_class: Rc<RefCell<Option<Rc<ClassCompiler>>>>,
    pub locals: Vec<Local>,
    pub scope_depth: usize,
}

impl Compiler {
    pub fn new(kind: FunctionKind) -> Self {
        let mut result = Self {
            parser: Rc::new(RefCell::new(Parser::new())),
            scanner: Rc::new(RefCell::new(Scanner::new(""))),
            rules: Rc::new(Rules::new()),
            function: FunctionObject::new(),
            current_class: Rc::new(RefCell::new(None)),
            kind,
            locals: {
                let mut locals = Vec::new();
                locals.push(Local::new(
                    Token {
                        kind: TokenKind::Identifier,
                        lexeme: "".to_string(),
                        line_number: 1,
                    },
                    Some(0),
                ));
                locals
            },
            scope_depth: 0,
        };
        if kind != FunctionKind::Script {
            let name = result.parser().previous.lexeme.clone();
            result.function.name = name;
        }
        if kind != FunctionKind::Function {
            result.locals[0].name.lexeme = "self".to_string();
        } else {
            result.locals[0].name.lexeme = "".to_string();
        }

        result
    }

    pub fn fork(&self, kind: FunctionKind) -> Self {
        let mut result = Self {
            parser: self.parser.clone(),
            scanner: self.scanner.clone(),
            rules: self.rules.clone(),
            function: FunctionObject::new(),
            kind,
            current_class: self.current_class.clone(),
            locals: {
                let mut locals = Vec::new();
                locals.push(Local::new(
                    Token {
                        kind: TokenKind::Identifier,
                        lexeme: "".to_string(),
                        line_number: 1,
                    },
                    Some(0),
                ));
                locals
            },
            scope_depth: 0,
        };
        if kind != FunctionKind::Script {
            let name = result.parser().previous.lexeme.clone();
            result.function.name = name;
        }
        if kind != FunctionKind::Function {
            result.locals[0].name.lexeme = "self".to_string();
        } else {
            result.locals[0].name.lexeme = "".to_string();
        }

        result
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

    pub fn compile(mut self, source: &str) -> Result<FunctionObject, InterpretError> {
        *self.scanner() = Scanner::new(source);
        self.advance();
        while !self.matches(TokenKind::EOF) {
            self.parse_declaration();
        }
        if self.parser().had_error.get() {
            Err(InterpretError::CompileError)
        } else {
            let function = self.end_complier();
            Ok(function)
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

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.function.chunk
    }

    fn emit_one_byte<T: Into<u8>>(&mut self, byte: T) {
        let line_number = self.parser().previous.line_number;
        self.current_chunk().push_bytecode(byte.into(), line_number);
    }

    fn emit_two_bytes<T: Into<u8>, U: Into<u8>>(&mut self, byte1: T, byte2: U) {
        self.emit_one_byte(byte1.into());
        self.emit_one_byte(byte2.into());
    }

    fn emit_return(&mut self) {
        use OpCode::*;

        if self.kind == FunctionKind::Initializer {
            self.emit_two_bytes(GetLocal, 0);
        } else {
            self.emit_one_byte(None);
        }
        self.emit_one_byte(Return);
    }

    fn end_complier(mut self) -> FunctionObject {
        self.emit_return();
        if !self.parser().had_error.get() {
            self.current_chunk().disassemble_chunk("<script>");
        }
        self.function
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
        if let Some(index) = self.current_chunk().push_constant(value) {
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
        if self.locals.len() == u8::MAX.into() {
            self.parser().error("Too many local variables in function.");
            return;
        }
        self.locals.push(Local::new(name, None));
    }
}
