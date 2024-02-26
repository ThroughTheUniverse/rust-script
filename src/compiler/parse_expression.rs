use super::{parse_rule::Precedence, Compiler};

impl Compiler {
    pub fn parse_expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }
}
