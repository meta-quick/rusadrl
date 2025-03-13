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
use crate::model::constraint::{ConstraintUnion};
use crate::model::party::Party;
use crate::model::stateworld::StateWorld;
use crate::traits::definions::LogicEval;

//http://www.w3.org/ns/odrl/2/Rule
#[derive(Debug,Builder,Getter,GetterMut,Setter,Default,Clone)]
pub struct Rule {
    pub uid: Option<IriBuf>,
    pub action: Option<Action>,
    pub target: Option<Asset>,
    pub constraint: Option<Vec<ConstraintUnion>>,
    pub assignee: Option<Party>,
    pub assigner: Option<Party>,
    pub output: Option<Asset>,
    pub relation: Option<IriBuf>,
    pub function: String,
    pub failure: String,
    pub metadata: Metadata,
}

pub struct RuleInference;

impl RuleInference {
    pub fn infer(candidate: Rule, world: &StateWorld) -> Result<bool,anyhow::Error> {
        let mut result = true;
        if let Some(constraints ) = &candidate.get_constraint() {
            for constraint in constraints {
                match constraint {
                    ConstraintUnion::Constraint(c) => {
                        let mut world = world.clone();
                        let ret = c.eval(&mut world);
                        match ret {
                           Ok(false) => { result = false; },
                           _ => {
                           }
                        }
                    }
                    ConstraintUnion::LogicConstraint(lc) => {
                        let mut world = world.clone();
                        let ret = lc.eval(&mut world);
                        match ret {
                           Ok(false) => { result = false; },
                           _ => {
                           }
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}