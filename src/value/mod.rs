use std::rc::Rc;

use crate::object::{
    bound_method::BoundMethod, closure::Closure, function::Function, instance::Instance,
    native_function::NativeFunction, r#struct::Struct,
};

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Function(Rc<Function>),
    NativeFunction(Rc<dyn NativeFunction>),
    Closure(Rc<Closure>),
    Struct(Rc<Struct>),
    Instance(Rc<Instance>),
    Bound(Rc<BoundMethod>),
    None,
}

pub struct Constants {
    pub values: Vec<Value>,
}

impl Constants {
    pub fn new() -> Self {
        Constants { values: Vec::new() }
    }

    pub fn write_value(&mut self, value: Value) -> usize {
        let index = self.values.len();
        self.values.push(value);
        index
    }
}
