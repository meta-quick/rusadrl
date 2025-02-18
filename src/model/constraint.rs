#![allow(dead_code)]

use crate::model::metadata::Metadata;

#[derive(Debug,Default, Clone)]
pub struct Constraint {
    pub metadata: Metadata,
    pub operator: String,
    pub left_operand: String,
    pub right_operand: String,
}

impl Constraint {
    pub fn new() -> Self {
        Constraint {
            metadata: Metadata::new(),
            operator: String::new(),
            left_operand: String::new(),
            right_operand: String::new(),
        }
    }

    pub fn from(operator: String, left_operand: String, right_operand: String) -> Self {
        Constraint {
            metadata: Metadata::new(),
            operator,
            left_operand,
            right_operand,
        }
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_operator(&mut self, operator: String) {
        self.operator = operator;
    }

    pub fn set_left_operand(&mut self, left_operand: String) {
        self.left_operand = left_operand;
    }
    
    pub fn set_right_operand(&mut self, right_operand: String) {
        self.right_operand = right_operand;
    }
}