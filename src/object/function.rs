use std::{fmt::Display, rc::Rc};

use crate::chunk::Chunk;

pub struct Function {
    arity: usize,
    pub chunk: Rc<Chunk>,
    name: String,
    pub upvalue_count: usize,
}

impl Function {
    pub fn new<T: Into<String>>(
        arity: usize,
        chunk: &Rc<Chunk>,
        name: T,
        upvalue_count: usize,
    ) -> Self {
        Self {
            arity,
            chunk: Rc::clone(chunk),
            name: name.into(),
            upvalue_count,
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
