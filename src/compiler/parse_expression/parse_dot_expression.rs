use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_dot_expression(&mut self, can_assign: bool) {
        self.consume(TokenKind::Identifier, "Expect property name after '.'.");
        let name = self.parser().previous.lexeme.clone();
        let name = self.emit_identifier_constant(name);
        if can_assign && self.matches(TokenKind::Equal) {
            self.parse_expression();
            self.emit_two_bytes(OpCode::SetProperty, name);
        } else if self.matches(TokenKind::LeftParen) {
            let arg_count = self.argument_list();
            self.emit_two_bytes(OpCode::Invoke, name);
            self.emit_one_byte(arg_count);
        } else {
            self.emit_two_bytes(OpCode::GetProperty, name);
        }
    }
}
