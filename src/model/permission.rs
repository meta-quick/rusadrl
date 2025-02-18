#![allow(dead_code)]
#![warn(non_snake_case)]

use crate::model::duty::Duty;
use crate::model::rule::Rule;
use crate::reference::types::RuleType;


#[derive(Debug, Default, Clone)]
pub struct Permission {
    pub rule: Rule,
    pub duties: Vec<Duty>,
}

impl Permission {
    pub fn new() -> Permission {
        let mut permission = Permission::default();
        permission.rule.set_rule_type(RuleType::Permission);
        permission
    }

    pub fn with_uri(uri: String) -> Permission {
        let mut permission = Permission::default();
        permission.rule.set_rule_type(RuleType::Permission);
        permission.rule.set_uri(uri);
        permission
    }

    pub fn clear_duties(&mut self) {
        self.duties.clear();
    }

    pub fn add_duty(&mut self, duty: Duty) {
        self.duties.push(duty);
    }

    pub fn add_duties(&mut self, duties: Vec<Duty>) {
        self.duties.extend(duties);
    }

    pub fn get_duties(&self) -> &Vec<Duty> {
        &self.duties
    }
}

