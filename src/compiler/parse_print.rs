use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_print(&mut self) {
        self.parse_expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit_one_byte(OpCode::Print);
    }
}
