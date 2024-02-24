use crate::object::{
    bound_method::BoundMethod, closure::Closure, function::Function, instance::Instance,
    native_function::NativeFunction, r#struct::Struct,
};
use std::{
    any::Any,
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
};

#[derive(Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Function(Rc<Function>),
    NativeFunction(Rc<dyn NativeFunction>),
    Closure(Rc<Closure>),
    Struct(Rc<Struct>),
    Instance(Rc<Instance>),
    BoundMethod(Rc<BoundMethod>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            None => write!(f, "none"),
            Bool(bool) => write!(f, "{}", bool),
            Number(number) => write!(f, "{}", number),
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
            (Bool(a), Bool(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a.cmp(b) == Ordering::Equal,
            (Function(a), Function(b)) => Rc::ptr_eq(a, b),
            (NativeFunction(a), NativeFunction(b)) => a.type_id() == b.type_id(),
            (Struct(a), Struct(b)) => Rc::ptr_eq(a, b),
            (Instance(a), Instance(b)) => Rc::ptr_eq(a, b),
            (BoundMethod(a), BoundMethod(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        use Value::*;
        match self {
            Number(a) => Number(-a),
            _ => panic!("Only number can be negated"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a + b),
            _ => panic!("Only number can do addition"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a - b),
            _ => panic!("Only number can do substration"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a * b),
            _ => panic!("Only number can do multiplication"),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a / b),
            _ => panic!("Only number can do division"),
        }
    }
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
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
