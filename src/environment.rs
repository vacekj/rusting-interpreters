use std::collections::HashMap;

use crate::ast::LiteralValue;

pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> &LiteralValue {
        match self.values.get(&name) {
            Some(val) => val,
            None => {
                let error_message = format!("Undefined variable {}", name);
                panic!("{}", error_message);
            }
        }
    }

    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
}
