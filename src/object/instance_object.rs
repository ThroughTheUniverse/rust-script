use super::struct_object::StructObject;
use crate::value::Value;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

pub struct InstanceObject {
    pub r#struct: Rc<StructObject>,
    pub fields: RefCell<HashMap<String, Value>>,
}

impl InstanceObject {
    pub fn new(r#struct: Rc<StructObject>) -> Self {
        Self {
            r#struct: Rc::clone(&r#struct),
            fields: RefCell::new(HashMap::new()),
        }
    }
}

impl Display for InstanceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.r#struct)
    }
}
