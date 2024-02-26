use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_fn_call(&mut self, _can_assign: bool) {
        let arg_count = self.argument_list();
        self.emit_two_bytes(OpCode::Call, arg_count);
    }

    fn argument_list(&mut self) -> u8 {
        let mut arg_count = 0;
        if !self.check(TokenKind::RightParen) {
            loop {
                self.parse_expression();
                if arg_count == u8::MAX {
                    self.parser().error("Can't have more than 255 arguments.");
                }
                arg_count += 1;
                if !self.matches(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expect ')' after arguments.");
        arg_count
    }
}
