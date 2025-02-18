#![allow(dead_code)]
#![warn(non_snake_case)]

use crate::model::rule::Rule;
use crate::reference::types::RuleType;

#[derive(Debug, Default, Clone)]
pub struct Prohibition {
    pub rule: Rule,
}

impl Prohibition {
    pub fn new() -> Prohibition {
        let mut prohibition = Prohibition::default();
        prohibition.rule.set_rule_type(RuleType::Prohibition);
        prohibition
    }
    
    pub fn with_uri(uri: String) -> Prohibition {
        let mut prohibition = Prohibition::default();
        prohibition.rule.set_rule_type(RuleType::Prohibition);
        prohibition.rule.set_uri(uri);
        prohibition
    }
}