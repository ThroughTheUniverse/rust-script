use std::{fmt::Display, rc::Rc};

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
    BoundMethod(Rc<BoundMethod>),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Bool(bool) => write!(f, "{}", bool),
            Number(number) => write!(f, "{}", number),
            None => write!(f, "none"),
            String(string) => write!(f, "{}", string),
            Function(function) => write!(f, "{}", function),
            NativeFunction(_) => write!(f, "<native fn>"),
            Closure(closure) => write!(f, "{}", closure),
            Struct(r#struct) => write!(f, "{}", r#struct),
            Instance(instance) => write!(f, "{}", instance),
            BoundMethod(bound_method) => write!(f, "{}", bound_method),
        }
    }
}

impl Value {
    pub fn print(&self) {
        print!("{}", self);
    }
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
