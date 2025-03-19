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
#![allow(non_snake_case)]

use anyhow::anyhow;
use iref::IriBuf;
use lombok::{Getter,Builder,Setter,GetterMut};
use crate::model::constraint::ConstraintUnion;
use crate::model::metadata::Metadata;
use crate::model::stateworld::StateWorld;
use crate::traits::definions::LogicEval;

#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct PartyCollection {
    pub source: Option<IriBuf>,
    pub refinement: Option<Vec<ConstraintUnion>>,
    pub metadata: Metadata,
}

#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct Party {
    pub uid: Option<IriBuf>,
    //link to PartCollection
    pub partOf: Vec<IriBuf>,
    pub refinement: Option<Vec<ConstraintUnion>>,
    //linked to Policy
    pub assignerOf: Option<IriBuf>,
    //linked to Policy
    pub assigneeOf: Option<IriBuf>,
    pub metadata: Metadata,
}


impl Party {
    pub fn new() -> Self {
        Party {
            uid: None,
            metadata: Metadata::new(),
            partOf: Vec::new(),
            refinement: None,
            assignerOf: None,
            assigneeOf: None,
        }
    }
}

#[derive(Debug,Clone)]
pub enum PartyUnion {
    Party(Party),
    PartyCollection(PartyCollection),
}

#[derive(Debug,Default,Clone)]
pub struct PartyInferencer;

impl PartyInferencer {
    pub fn infer_party(world: &mut StateWorld, party: &PartyUnion,candidate: &Party) -> Result<bool, anyhow::Error>{
        match party {
            PartyUnion::Party(party) => {
                let candidate_uid = candidate.get_uid();
                if let None = candidate_uid {
                    return Err(anyhow!("Party uid is None"));
                }
                let candidate_uid = candidate_uid.clone().unwrap();
                let candidate_uid = candidate_uid.as_str();

                let self_uid = party.get_uid();
                if let None = self_uid {
                    return Err(anyhow!("Party uid is None"));
                }
                let self_uid = self_uid.clone().unwrap();
                let self_uid = self_uid.as_str();

                if candidate_uid == self_uid {
                    //check refinement
                    if let Some(refinement) = party.get_refinement() {
                        let mut refined = false;
                        for constraint in refinement {
                            match constraint {
                                ConstraintUnion::Constraint(constraint) => {
                                    let ret = constraint.eval(world);
                                    match ret {
                                        Ok(true) => {
                                            refined = true;
                                        },
                                        _ => {
                                        }
                                    }
                                }
                                ConstraintUnion::LogicConstraint(ac) => {
                                    let ret = ac.eval(world);
                                    match ret {
                                        Ok(true) => {
                                            refined = true;
                                        },
                                        _ => {
                                        }
                                    }
                                }
                            }
                        }
                        return Ok(refined);
                    } else {
                        return Ok(true);
                    }
                }
            }
            PartyUnion::PartyCollection(partyCollect) => {
                //check if candidate uid equals to partyCollection source
                let candidate_uid = candidate.get_uid();
                if let None = candidate_uid {
                    return Err(anyhow!("Party uid is None"));
                }
                let candidate_uid = candidate_uid.clone().unwrap();
                let candidate_uid = candidate_uid.as_str();

                if let Some(source) =  partyCollect.get_source() {
                    let source = source.as_str();
                    if candidate_uid == source {
                        //check refinement
                        if let Some(refinement) = partyCollect.get_refinement() {
                            let mut refined = false;
                            for constraint in refinement {
                                match constraint {
                                    ConstraintUnion::Constraint(constraint) => {
                                        let ret = constraint.eval(world);
                                        match ret {
                                            Ok(true) => {
                                                refined = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                    ConstraintUnion::LogicConstraint(ac) => {
                                        let ret = ac.eval(world);
                                        match ret {
                                            Ok(true) => {
                                                refined = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            return Ok(refined);
                        }
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
}