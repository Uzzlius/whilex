use std::collections::HashMap;

use num_bigint::BigUint;

pub struct Environment {
    vars: HashMap<String, BigUint>,
}

impl Environment {
    pub fn new(init: Vec<BigUint>) -> Self {
        let mut environment = Self {
            vars: HashMap::new(),
        };

        for (index, num) in init.into_iter().enumerate() {
            environment.assign(&format!("v{}", index), num);
        }

        environment
    }

    pub fn assign(&mut self, name: &str, num: BigUint) {
        self.vars.insert(name.to_string(), num);
    }

    pub fn retrieve(&self, name: &str) -> &BigUint {
        self.vars.get(name).unwrap_or(&BigUint::ZERO)
    }
}
