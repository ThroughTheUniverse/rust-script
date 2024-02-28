use super::Compiler;
use crate::{chunk::opcode::OpCode, compiler::FunctionKind, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_return_statement(&mut self) {
        use FunctionKind::*;
        use TokenKind::*;

        if self.kind == Script {
            self.parser().error("Can't return from top-level code.");
        }

        if self.matches(Semicolon) {
            self.emit_return();
        } else {
            if self.kind == Initializer {
                self.parser()
                    .error("Can't return a value from an initializer.");
            }
            self.parse_expression();
            self.consume(Semicolon, "Expect ';' after return value.");
            self.emit_one_byte(OpCode::Return);
        }
    }
}
