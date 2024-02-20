use std::rc::Rc;

use crate::chunk::Chunk;

pub struct Function {
    arity: usize,
    pub chunk: Rc<Chunk>,
    name: String,
    upvalue_count: usize,
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
