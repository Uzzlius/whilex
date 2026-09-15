use std::collections::HashMap;

use num_bigint::BigUint;

use crate::grammar::*;

pub struct Environment<'a> {
    vars: HashMap<String, BigUint>,
    procs: HashMap<String, &'a Stmt<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(init: Vec<BigUint>) -> Self {
        let mut environment = Self {
            vars: HashMap::new(),
            procs: HashMap::new(),
        };

        for (index, num) in init.into_iter().enumerate() {
            environment.assign_var(&format!("x{}", index), num);
        }

        environment
    }

    pub fn assign_var(&mut self, name: &str, num: BigUint) {
        self.vars.insert(name.to_string(), num);
    }

    pub fn retrieve_var(&self, name: &str) -> &BigUint {
        self.vars.get(name).unwrap_or(&BigUint::ZERO)
    }

    pub fn assign_proc(&mut self, name: &str, proc: &'a Stmt<'a>) {
        self.procs.insert(name.to_string(), proc);
    }

    pub fn retrieve_proc(&self, name: &str) -> Option<&'a Stmt<'a>> {
        self.procs.get(name).copied()
    }
}
