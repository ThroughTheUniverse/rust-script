use crate::object::{
    bound_method::BoundMethod, closure::Closure, function::Function, instance::Instance,
    native_function::NativeFunction, r#struct::Struct,
};
use std::{any::Any, cmp::Ordering, fmt::Display, rc::Rc};

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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (None, None) => true,
            (String(a), String(b)) => a.cmp(b) == Ordering::Equal,
            (Function(a), Function(b)) => Rc::ptr_eq(a, b),
            (Struct(a), Struct(b)) => Rc::ptr_eq(a, b),
            (NativeFunction(a), NativeFunction(b)) => a.type_id() == b.type_id(),
            (Number(a), Number(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Instance(a), Instance(b)) => Rc::ptr_eq(a, b),
            (BoundMethod(a), BoundMethod(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

pub struct ConstantPool(pub Vec<Value>);

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool(Vec::new())
    }

    pub fn print_nth(&self, index: usize) {
        print!("{}", self.0.get(index).unwrap());
    }

    pub fn get(&self, index: usize) -> &Value {
        &self.0[index]
    }

    pub fn push(&mut self, value: Value) -> usize {
        let index = self.0.len();
        self.0.push(value);
        index
    }
}
