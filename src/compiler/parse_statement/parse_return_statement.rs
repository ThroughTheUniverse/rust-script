use crate::{chunk::opcode::OpCode, compiler::FunctionKind, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_return_statement(&mut self) {
        if self.kind == FunctionKind::Script {
            self.parser().error("Can't return from top-level code.");
        }

        if self.matches(TokenKind::Semicolon) {
            self.emit_return();
        } else {
            if self.kind == FunctionKind::Initializer {
                self.parser()
                    .error("Can't return a value from an initializer.");
            }
            self.parse_expression();
            self.consume(TokenKind::Semicolon, "Expect ';' after return value.");
            self.emit_one_byte(OpCode::Return);
        }
    }
}
