use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::{Compiler, FunctionKind};

impl Compiler {
    pub fn parse_return(&mut self) {
        if self.kind == FunctionKind::Script {
            self.parser().error("Can't return from top-level code.");
        }

        if self.matches(TokenKind::Semicolon) {
            self.emit_return();
        } else {
            self.parse_expression();
            self.consume(TokenKind::Semicolon, "Expect ';' after return value.");
            self.emit_one_byte(OpCode::Return);
        }
    }
}
