use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_identifier(&mut self, can_assign: bool) {
        let name = self.parser.previous.lexeme.to_string();
        self.named_identifier(name, can_assign);
    }

    fn named_identifier(&mut self, name: String, can_assign: bool) {
        let arg = self.emit_identifier_constant(name);
        if can_assign && self.matches(TokenKind::Equal) {
            self.parse_expression();
            self.emit_two_bytes(OpCode::SetGlobal, arg);
        } else {
            self.emit_two_bytes(OpCode::GetGlobal, arg);
        }
    }
}
