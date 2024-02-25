use crate::scanner::token::Token;

pub struct Local {
    pub name: Token,
    pub depth: Option<usize>,
}

impl Local {
    pub fn new(name: Token, depth: Option<usize>) -> Self {
        Self { name, depth }
    }
}
