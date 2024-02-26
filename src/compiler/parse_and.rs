use crate::chunk::opcode::OpCode;

use super::{parse_rule::Precedence, Compiler};

impl Compiler {
    pub fn parse_and(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_one_byte(OpCode::Pop);
        self.parse_precedence(Precedence::And);
        self.patch_jump(end_jump);
    }
}
