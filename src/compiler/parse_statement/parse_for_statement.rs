use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_for_statement(&mut self) {
        use OpCode::*;
        use TokenKind::*;

        self.begin_scope();
        self.consume(LeftParen, "Expect '(' after 'for'.");

        if self.matches(Semicolon) {
        } else if self.matches(Let) {
            self.parse_let_declaration();
        } else {
            self.parse_expression_statement();
        }

        let prev_loop_start = self.loop_start;
        let prev_loop_depth = self.loop_depth;
        self.loop_start = Some(self.current_chunk().bytecodes.len()); // start condition
        self.loop_depth = self.scope_depth;

        let mut exit_jump = Option::None;
        if !self.matches(Semicolon) {
            self.parse_expression();
            self.consume(Semicolon, "Expect ';' after loop condition.");
            exit_jump = Some(self.emit_jump(JumpIfFalse));
            self.emit_one_byte(Pop);
        }

        if !self.matches(RightParen) {
            let body_jump = self.emit_jump(Jump);

            let increment_start = self.current_chunk().bytecodes.len();

            self.parse_expression();
            self.emit_one_byte(Pop);
            self.consume(RightParen, "Expect ')' after for clauses.");
            self.emit_loop(self.loop_start.unwrap());
            self.loop_start = Some(increment_start);
            self.patch_jump(body_jump);
        }

        self.parse_statement();
        self.emit_loop(self.loop_start.unwrap());
        self.end_loop();
        self.loop_start = prev_loop_start;
        self.loop_depth = prev_loop_depth;

        if exit_jump.is_some() {
            self.patch_jump(exit_jump.unwrap());
            self.emit_one_byte(Pop);
        }

        self.end_scope();
    }
}
