use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_while_statement(&mut self) {
        use OpCode::*;
        use TokenKind::*;

        let loop_start = self.current_chunk().bytecodes.len();
        self.consume(LeftParen, "Expect '(' after 'while'.");
        self.parse_expression();
        self.consume(RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(JumpIfFalse);
        self.emit_one_byte(Pop);
        self.parse_statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_one_byte(Pop);
    }

    pub fn emit_loop(&mut self, loop_start: usize) {
        self.emit_one_byte(OpCode::Loop);
        let offset = self.current_chunk().bytecodes.len() - loop_start + 2;
        if offset > u16::MAX.into() {
            self.parser().error("Loop body too large.");
        }

        self.emit_one_byte(((offset >> 8) & 0xff) as u8);
        self.emit_one_byte((offset & 0xff) as u8);
    }
}
