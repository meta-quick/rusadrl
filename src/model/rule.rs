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
use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};

use crate::model::metadata::Metadata;
use crate::model::action::Action;
use crate::model::constraint::Constraint;
use crate::model::party::Party;
use crate::reference::types::RuleType;

#[derive(Debug,Builder,Getter,GetterMut,Setter,Default,Clone)]
pub struct Rule {
    pub metadata: Metadata,
    pub kind: RuleType,
    pub actions: Vec<Action>,
    pub constraint: Vec<Constraint>,
    pub consequence: Vec<Rule>,
    pub remedy: Vec<Rule>,
    pub target: Option<IriBuf>,
    pub assignee: Option<Party>,
    pub assigner: Option<Party>,
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
            target: None,
            assignee: None,
            assigner: None,
        }
    }
}