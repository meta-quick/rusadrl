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

use lombok::{Builder,Setter,GetterMut,Getter};
use crate::model::action::Action;
use crate::model::asset::{AssetUnion};
use crate::model::constraint::{ConstraintUnion};
use crate::model::party::{PartyUnion};
use crate::model::rule::Rule;



#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Permission {
    pub duty: Rule,
}

impl Permission {
    pub fn set_assignee(&mut self, assignee: Option<PartyUnion>) {
        self.duty.set_assignee(assignee);
    }

    pub fn get_assignee(&self) -> &Option<PartyUnion> {
        self.duty.get_assignee()
    }

    pub fn set_assigner(&mut self, assigner: Option<PartyUnion>) {
        self.duty.set_assigner(assigner);
    }

    pub fn get_assigner(&self) -> &Option<PartyUnion> {
        self.duty.get_assigner()
    }

    pub fn get_target(&self) -> &Option<AssetUnion> {
        self.duty.get_target()
    }

    pub fn get_constraint(&self) -> &Option<Vec<ConstraintUnion>> {
        self.duty.get_constraint()
    }

    pub fn get_action(&self) -> &Option<Action> {
        self.duty.get_action()
    }
}