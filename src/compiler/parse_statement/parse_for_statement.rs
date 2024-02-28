use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_for_statement(&mut self) {
        self.begin_scope();
        self.consume(TokenKind::LeftParen, "Expect '(' after 'for'.");

        if self.matches(TokenKind::Semicolon) {
        } else if self.matches(TokenKind::Let) {
            self.parse_let_declaration();
        } else {
            self.parse_expression_statement();
        }

        let mut loop_start = self.current_chunk().bytecodes.len();
        let mut exit_jump = None;
        if !self.matches(TokenKind::Semicolon) {
            self.parse_expression();
            self.consume(TokenKind::Semicolon, "Expect ';' after loop condition.");
            exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse));
            self.emit_one_byte(OpCode::Pop);
        }

        if !self.matches(TokenKind::RightParen) {
            let body_jump = self.emit_jump(OpCode::Jump);
            let increment_start = self.current_chunk().bytecodes.len();
            self.parse_expression();
            self.emit_one_byte(OpCode::Pop);
            self.consume(TokenKind::RightParen, "Expect ')' after for clauses.");
            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        self.parse_statement();
        self.emit_loop(loop_start);

        if exit_jump.is_some() {
            self.patch_jump(exit_jump.unwrap());
            self.emit_one_byte(OpCode::Pop);
        }

        self.end_scope();
    }
}
