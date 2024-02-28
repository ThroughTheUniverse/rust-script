use crate::{chunk::opcode::OpCode, compiler::parser::parse_rule::Precedence};

use super::Compiler;

impl Compiler {
    pub fn parse_and_expression(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_one_byte(OpCode::Pop);
        self.parse_precedence(Precedence::And);
        self.patch_jump(end_jump);
    }
}
