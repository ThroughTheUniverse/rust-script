use crate::object::{
    bound_method_object::BoundMethodObject, function_object::FunctionObject,
    instance_object::InstanceObject, native_function_object::NativeFunctionObject,
    struct_object::StructObject,
};
use std::{
    any::Any,
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
};

pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Function(Rc<FunctionObject>),
    NativeFunction(Rc<dyn NativeFunctionObject>),
    Struct(Rc<StructObject>),
    Instance(Rc<InstanceObject>),
    BoundMethod(Rc<BoundMethodObject>),
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
            Struct(r#struct) => write!(f, "{}", r#struct),
            Instance(instance) => write!(f, "{}", instance),
            BoundMethod(bound_method) => write!(f, "{}", bound_method),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        use Value::*;
        match self {
            None => None,
            Bool(b) => Bool(*b),
            Number(n) => Number(*n),
            String(s) => String(s.clone()),
            Function(f) => Function(Rc::clone(f)),
            NativeFunction(n) => NativeFunction(Rc::clone(n)),
            Struct(c) => Struct(Rc::clone(c)),
            Instance(i) => Instance(Rc::clone(i)),
            BoundMethod(b) => BoundMethod(Rc::clone(b)),
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

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
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
            (Value::String(a), Value::String(b)) => Value::String(a + &b),
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
    pub fn modulo(self, rhs: Self) -> Self {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a.rem_euclid(b)),
            _ => panic!("Only number can do modulo"),
        }
    }

    pub fn power(self, rhs: Self) -> Self {
        use Value::*;
        match (self, rhs) {
            (Number(a), Number(b)) => Number(a.powf(b)),
            _ => panic!("Only number can do power"),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        if let Value::None = self {
            return true;
        }

        if let Value::Bool(boolean) = self {
            !boolean
        } else {
            false
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
