use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
