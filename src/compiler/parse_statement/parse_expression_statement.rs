use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_expression_statement(&mut self) {
        self.parse_expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.");
        self.emit_one_byte(OpCode::Pop);
    }
}
