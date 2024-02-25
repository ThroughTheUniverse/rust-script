use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_let(&mut self) {
        let global = self.parse_variable("Expect variable name.");
        if self.matches(TokenKind::Equal) {
            self.parse_expression();
        } else {
            self.emit_one_byte(OpCode::None);
        }
        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        );
        self.define_variable(global);
    }

    fn parse_variable(&mut self, error_message: &str) -> u8 {
        self.consume(TokenKind::Identifier, error_message);
        let name = self.parser.previous.lexeme.clone();
        self.emit_identifier_constant(name)
    }

    fn define_variable(&mut self, global: u8) {
        self.emit_two_bytes(OpCode::DefineGlobal, global);
    }
}
