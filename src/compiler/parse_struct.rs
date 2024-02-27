use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_struct(&mut self) {
        self.consume(TokenKind::Identifier, "Expect class name.");
        let struct_name = self.parser().previous.lexeme.clone();
        let name_constant = self.emit_identifier_constant(struct_name.clone());
        self.declare_variable();
        self.emit_two_bytes(OpCode::Struct, name_constant);
        self.define_variable(name_constant);
        self.named_identifier(struct_name, false);
        self.consume(TokenKind::LeftBrace, "Expect '{' before class body.");
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
            self.parse_method();
        }
        self.consume(TokenKind::RightBrace, "Expect '}' before class body.");
        self.emit_one_byte(OpCode::Pop);
    }

    fn parse_method(&mut self) {
        self.consume(TokenKind::Identifier, "Expect method name.");
        let name = self.parser().previous.lexeme.clone();
        let constant = self.emit_identifier_constant(name);
        self.parse_function(super::FunctionKind::Function);
        self.emit_two_bytes(OpCode::Method, constant);
    }
}
