use super::{parse_rule::Precedence, Compiler};

impl<'a> Compiler<'a> {
    pub fn parse_expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }
}
