#![allow(dead_code)]

use crate::model::metadata::Metadata;
use crate::model::action::Action;
use crate::model::constraint::Constraint;
use crate::model::party::Party;
use crate::reference::types::RuleType;

#[derive(Debug,Default, Clone)]
pub struct Rule {
    pub metadata: Metadata,
    pub kind: RuleType,
    pub actions: Vec<Action>,
    pub constraint: Vec<Constraint>,
    pub consequence: Vec<Rule>,
    pub remedy: Vec<Rule>,
    pub target: String,
    pub assignee: Party,
    pub assigner: Party,
}

impl Rule {
    pub fn new() -> Self {
        Rule {
            metadata: Metadata::new(),
            kind: RuleType::Permission,
            actions: Vec::new(),
            constraint: Vec::new(),
            consequence: Vec::new(),
            remedy: Vec::new(),
            target: String::new(),
            assignee: Party::new(),
            assigner: Party::new(),
        }
    }

    pub fn set_rule_type(&mut self, kind: RuleType) {
        self.kind = kind;
    }

    pub fn set_uri(&mut self, uri: String) {
        self.metadata.set_uri(uri);
    }

    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    pub fn set_assignee(&mut self, assignee: Party) {
        self.assignee = assignee;
    }

    pub fn set_assigner(&mut self, assigner: Party) {
        self.assigner = assigner;
    }
}