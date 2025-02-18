// Copyright 2024 meduo <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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