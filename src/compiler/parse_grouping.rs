use crate::scanner::token::TokenKind;

use super::Compiler;

impl Compiler {
    pub fn parse_grouping(&mut self, _can_assign: bool) {
        self.parse_expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression");
    }
}
