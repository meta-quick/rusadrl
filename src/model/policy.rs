#![allow(dead_code)]
use crate::model::metadata::Metadata;
use crate::model::rule::Rule;
use crate::model::party::Party;
use crate::reference::types::ReferenceType;
use crate::reference::types::RuleType;

#[derive(Debug,Default, Clone)]
pub struct Policy {
    pub metadata: Metadata,
    pub file_name: String,
    pub rules : Vec<Rule>,
    pub profile: Vec<String>,
    pub target : Vec<String>,
    pub inherit_from: Vec<String>,
    pub conflict: String,
    pub type_of_policy: String,
}

impl Policy {
    pub fn new() -> Self {
        Policy {
            metadata: Metadata::new(),
            file_name: String::new(),
            rules: Vec::new(),
            profile: Vec::new(),
            target: Vec::new(),
            inherit_from: Vec::new(),
            conflict: String::new(),
            type_of_policy: String::new(),
        }
    }

    pub fn with_uri(uri: String) -> Self {
        let mut policy = Policy::default();
        policy.metadata.set_uri(uri);
        policy
    }

    pub fn type_of_policy(&self) -> String {
        self.type_of_policy.clone()
    }

    pub fn set_type_of_policy(&mut self, type_of_policy: String) {
        self.type_of_policy = type_of_policy;
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn add_rules(&mut self, rules: Vec<Rule>) {
        self.rules.extend(rules);
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    pub fn set_rule_target(&mut self, target: &str) {
        for rule in &mut self.rules {
            rule.set_target(target.to_string());
        }
    }

    pub fn set_rule_assignee(&mut self, assignee: Party) {
        for rule in &mut self.rules {
            rule.set_assignee(assignee.clone());
        }
    }

    pub fn set_rule_assigner(&mut self, assigner: Party) {
        for rule in &mut self.rules {
            rule.set_assigner(assigner.clone());
        }
    }

    pub fn context(&self) -> String {
        let context = ReferenceType::PolicySet;
        context.as_str().to_owned()
    }

    pub fn permissions(&self) -> Vec<Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.kind == RuleType::Permission)
            .cloned()
            .collect()
    }

    pub fn prohibitions(&self) -> Vec<Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.kind == RuleType::Prohibition)
            .cloned()
            .collect()
    }

    pub fn duties(&self) -> Vec<Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.kind == RuleType::Duty)
            .cloned()
            .collect()
    }
}