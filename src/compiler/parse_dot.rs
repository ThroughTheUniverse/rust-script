use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_dot(&mut self, can_assign: bool) {
        self.consume(TokenKind::Identifier, "Expect property name after '.'.");
        let name = self.parser().previous.lexeme.clone();
        let name = self.emit_identifier_constant(name);
        if can_assign && self.matches(TokenKind::Equal) {
            self.parse_expression();
            self.emit_two_bytes(OpCode::SetProperty, name);
        } else {
            self.emit_two_bytes(OpCode::GetProperty, name);
        }
    }
}
