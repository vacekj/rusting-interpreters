use std::collections::HashMap;

use crate::scanner::TokenValue;

struct Environment {
    values: HashMap<String, TokenValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: TokenValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> TokenValue {
        match self.values.get(&name) {
            Some(val) => val.to_owned(),
            None => {
                let error_message = format!("Undefined variable {}", name);
                panic!("{}", error_message);
            }
        }
    }
}
