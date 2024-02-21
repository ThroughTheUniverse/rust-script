use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::closure::Closure;

pub struct Struct {
    name: String,
    methods: RefCell<HashMap<String, Rc<Closure>>>,
    init: RefCell<Option<Rc<Closure>>>,
}

impl Struct {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: RefCell::new(HashMap::new()),
            init: RefCell::new(None),
        }
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
