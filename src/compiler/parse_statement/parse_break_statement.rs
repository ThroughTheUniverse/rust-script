use crate::{chunk::opcode::OpCode, compiler::Compiler, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_break_statement(&mut self) {
        if self.loop_start.is_none() {
            self.parser().error("Cannot use 'break' outside of a loop.");
        }
        self.consume(TokenKind::Semicolon, "Expect ';' after 'break'.");
        self.emit_jump(OpCode::End);
    }
}
