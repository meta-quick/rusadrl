#![allow(dead_code)]
#![warn(non_snake_case)]

use crate::reference::types::RuleType;


#[derive(Debug, Default, Clone)]
pub struct Duty {
    pub rule: super::rule::Rule,
}

impl Duty {
    pub fn new() -> Duty {
        let mut duty =  Duty::default();
        
        duty.rule.set_rule_type(RuleType::Duty);
        duty
    }

    pub fn with_uri(uri: String) -> Duty {
        let mut duty =  Duty::default();

        duty.rule.set_rule_type(RuleType::Duty);
        duty.rule.set_uri(uri);
        duty
    }
}