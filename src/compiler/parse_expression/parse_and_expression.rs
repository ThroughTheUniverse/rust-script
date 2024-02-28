use super::Compiler;
use crate::{chunk::opcode::OpCode, compiler::parser::parse_rule::Precedence};

impl Compiler {
    pub fn parse_and_expression(&mut self, _can_assign: bool) {
        use OpCode::*;

        let end_jump = self.emit_jump(JumpIfFalse);
        self.emit_one_byte(Pop);
        self.parse_precedence(Precedence::And);
        self.patch_jump(end_jump);
    }
}
