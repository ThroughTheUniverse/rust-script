use crate::{chunk::opcode::OpCode, compiler::parser::parse_rule::Precedence};

use super::Compiler;

impl Compiler {
    pub fn parse_or_expression(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::JumpIfFalse);
        let end_jump = self.emit_jump(OpCode::Jump);
        self.patch_jump(else_jump);
        self.emit_one_byte(OpCode::Pop);
        self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
    }
}
