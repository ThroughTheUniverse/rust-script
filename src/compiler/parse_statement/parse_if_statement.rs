use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_if_statement(&mut self) {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'.");
        self.parse_expression();
        self.consume(TokenKind::RightParen, "Expect ')' after condition.");
        let then_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_one_byte(OpCode::Pop);
        self.parse_statement();
        let else_jump = self.emit_jump(OpCode::Jump);
        self.patch_jump(then_jump);
        self.emit_one_byte(OpCode::Pop);
        if self.matches(TokenKind::Else) {
            self.parse_statement();
        }
        self.patch_jump(else_jump);
    }

    pub fn emit_jump<T: Into<u8>>(&mut self, instruction: T) -> usize {
        self.emit_one_byte(instruction);
        self.emit_one_byte(u8::MAX);
        self.emit_one_byte(u8::MAX);
        self.current_chunk().bytecodes.len() - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.current_chunk().bytecodes.len() - offset - 2;

        if jump > u16::MAX.into() {
            self.parser().error("Too much code to jump over.");
        }

        self.current_chunk().bytecodes[offset] = ((jump >> 8) & 0xff) as u8;
        self.current_chunk().bytecodes[offset + 1] = (jump & 0xff) as u8;
    }
}
