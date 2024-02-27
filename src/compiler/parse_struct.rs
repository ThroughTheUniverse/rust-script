use std::rc::Rc;

use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::{ClassCompiler, Compiler, FunctionKind};

impl Compiler {
    pub fn parse_struct(&mut self) {
        self.consume(TokenKind::Identifier, "Expect struct name.");
        let struct_name = self.parser().previous.lexeme.clone();
        let name_constant = self.emit_identifier_constant(struct_name.clone());
        self.declare_variable();
        self.emit_two_bytes(OpCode::Struct, name_constant);
        self.define_variable(name_constant);

        let prev = self
            .current_class
            .replace(Some(Rc::new(ClassCompiler::new())));
        self.current_class
            .borrow()
            .as_ref()
            .unwrap()
            .enclosing
            .replace(prev);

        self.named_identifier(struct_name, false);
        self.consume(TokenKind::LeftBrace, "Expect '{' before struct body.");
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
            self.parse_method();
        }
        self.consume(TokenKind::RightBrace, "Expect '}' before struct body.");
        self.emit_one_byte(OpCode::Pop);

        let prev = self
            .current_class
            .borrow()
            .as_ref()
            .unwrap()
            .enclosing
            .replace(None);
        self.current_class.replace(prev);
    }

    fn parse_method(&mut self) {
        self.consume(TokenKind::Identifier, "Expect method name.");
        let name = self.parser().previous.lexeme.clone();
        let constant = self.emit_identifier_constant(name);
        let mut kind = FunctionKind::Method;
        if self.parser().previous.lexeme == "new" {
            kind = FunctionKind::Initializer;
        }
        self.parse_function(kind);
        self.emit_two_bytes(OpCode::Method, constant);
    }
}
