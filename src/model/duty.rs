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

use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::asset::Asset;
use crate::model::party::Party;
use super::rule::Rule;

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Duty {
    pub rule: Rule,
}

impl Duty {
    pub fn set_assignee(&mut self, assignee: Option<Party>) {
        self.rule.set_assignee(assignee);
    }

    pub fn get_assignee(&self) -> &Option<Party> {
        self.rule.get_assignee()
    }

    pub fn set_assigner(&mut self, assigner: Option<Party>) {
        self.rule.set_assigner(assigner);
    }

    pub fn get_assigner(&self) -> &Option<Party> {
        self.rule.get_assigner()
    }

    pub fn set_target(&mut self, target: Option<Asset>) {
        self.rule.set_target(target);
    }

    pub fn get_target(&self) -> &Option<Asset> {
        self.rule.get_target()
    }
}