use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_dot_expression(&mut self, can_assign: bool) {
        use OpCode::*;
        use TokenKind::*;

        self.consume(Identifier, "Expect property name after '.'.");
        let name = self.parser().previous.lexeme.clone();
        let name = self.emit_identifier_constant(name);
        if can_assign && self.matches(TokenKind::Equal) {
            self.parse_expression();
            self.emit_two_bytes(SetProperty, name);
        } else if self.matches(LeftParen) {
            let arg_count = self.argument_list();
            self.emit_two_bytes(Invoke, name);
            self.emit_one_byte(arg_count);
        } else {
            self.emit_two_bytes(GetProperty, name);
        }
    }
}
