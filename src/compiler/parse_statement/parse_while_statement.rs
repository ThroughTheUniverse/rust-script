use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_while_statement(&mut self) {
        use OpCode::*;
        use TokenKind::*;

        let prev_loop_start = self.loop_start;
        let prev_loop_depth = self.loop_depth;
        self.loop_start = Some(self.current_chunk().bytecodes.len());
        self.loop_depth = self.scope_depth;

        self.consume(LeftParen, "Expect '(' after 'while'.");
        self.parse_expression();
        self.consume(RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(JumpIfFalse);
        self.emit_one_byte(Pop);
        self.parse_statement();
        self.emit_loop(self.loop_start.unwrap());
        self.patch_jump(exit_jump);
        self.emit_one_byte(Pop);

        self.end_loop();
        self.loop_start = prev_loop_start;
        self.loop_depth = prev_loop_depth;
    }

    pub fn end_loop(&mut self) {
        let mut offset = self.loop_start.unwrap();
        while offset < self.current_chunk().bytecodes.len() {
            if self.current_chunk().bytecodes[offset] == OpCode::End.into() {
                self.current_chunk().bytecodes[offset] = OpCode::Jump.into();
                self.patch_jump(offset + 1);
            } else {
                offset += OpCode::from(self.current_chunk().bytecodes[offset]).to_offset() + 1;
            }
        }
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
