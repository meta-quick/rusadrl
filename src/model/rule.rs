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
use crate::model::asset::Asset;
use crate::model::constraint::Constraint;
use crate::model::party::Party;
use crate::reference::types::RuleType;

//http://www.w3.org/ns/odrl/2/Rule
#[derive(Debug,Builder,Getter,GetterMut,Setter,Default,Clone)]
pub struct Rule {
    pub uid: Option<IriBuf>,
    pub action: Action,
    pub target: Option<Party>,
    pub constraint: Vec<Constraint>,
    pub assignee: Option<Party>,
    pub assigner: Option<Party>,
    pub output: Option<Asset>,
    pub relation: Option<IriBuf>,
    pub function: String,
    pub failure: String,
    pub metadata: Metadata,
}