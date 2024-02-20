use std::{cell::RefCell, rc::Rc};

use super::{function::Function, upvalue::Upvalue};

pub struct Closure {
    function: Rc<Function>,
    upvalues: RefCell<Vec<Rc<Upvalue>>>,
}

impl Closure {
    pub fn new(function: Rc<Function>) -> Self {
        Self {
            function: Rc::clone(&function),
            upvalues: RefCell::new(Vec::new()),
        }
    }
}
