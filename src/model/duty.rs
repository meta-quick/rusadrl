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
#![warn(non_snake_case)]

use crate::reference::types::RuleType;


#[derive(Debug, Default, Clone)]
pub struct Duty {
    pub rule: super::rule::Rule,
}

impl Duty {
    pub fn new() -> Duty {
        let mut duty =  Duty::default();
        
        // duty.rule.set_rule_type(RuleType::Duty);
        duty
    }

    pub fn with_uri(uri: String) -> Duty {
        let mut duty =  Duty::default();

        // duty.rule.set_rule_type(RuleType::Duty);
        // duty.rule.set_uri(uri);
        duty
    }
}