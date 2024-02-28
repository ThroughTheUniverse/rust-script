use super::Compiler;
use crate::{chunk::opcode::OpCode, compiler::parser::parse_rule::Precedence};

impl Compiler {
    pub fn parse_or_expression(&mut self, _can_assign: bool) {
        use OpCode::*;

        let else_jump = self.emit_jump(JumpIfFalse);
        let end_jump = self.emit_jump(Jump);
        self.patch_jump(else_jump);
        self.emit_one_byte(Pop);
        self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
    }
}
