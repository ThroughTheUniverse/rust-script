use std::{fmt::Display, rc::Rc};

use crate::chunk::Chunk;

pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
    // pub upvalue_count: usize,
}

impl Function {
    pub fn new() -> Self {
        Self {
            arity: 0,
            chunk: Chunk::new(),
            name: "".to_string(),
            // upvalue_count,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            write!(f, "<script>")
        } else {
            write!(f, "<fn {}>", self.name)
        }
    }
}
