use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_while(&mut self) {
        let loop_start = self.chunk.bytecodes.len();
        self.consume(TokenKind::LeftParen, "Expect '(' after 'while'.");
        self.parse_expression();
        self.consume(TokenKind::RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_one_byte(OpCode::Pop);
        self.parse_statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_one_byte(OpCode::Pop);
    }

    pub fn emit_loop(&mut self, loop_start: usize) {
        self.emit_one_byte(OpCode::Loop);
        let offset = self.chunk.bytecodes.len() - loop_start + 2;
        if offset > u16::MAX.into() {
            self.parser.error("Loop body too large.");
        }

        self.emit_one_byte(((offset >> 8) & 0xff) as u8);
        self.emit_one_byte((offset & 0xff) as u8);
    }
}
