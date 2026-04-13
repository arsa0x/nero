use std::collections::HashMap;

use crate::interpreter::Value;

pub struct Resolver {
    pub variables: HashMap<String, Value>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
