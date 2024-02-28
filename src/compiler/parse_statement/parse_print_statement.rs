use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_print_statement(&mut self) {
        self.parse_expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit_one_byte(OpCode::Print);
    }
}
