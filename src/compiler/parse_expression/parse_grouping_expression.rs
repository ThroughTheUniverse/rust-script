use crate::scanner::token::TokenKind;

use super::Compiler;

impl Compiler {
    pub fn parse_grouping_expression(&mut self, _can_assign: bool) {
        self.parse_expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression");
    }
}
