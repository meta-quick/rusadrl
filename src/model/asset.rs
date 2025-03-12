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

use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};

use crate::model::metadata::Metadata;
use crate::model::stateworld::StateWorld;
use crate::traits::definions::LogicEval;
use super::constraint::{ConstraintUnion};

//http://www.w3.org/ns/odrl/2/AssetCollection
#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct AssetCollection {
    pub source  : Option<IriBuf>,
    pub refinement: Option<Vec<ConstraintUnion>>,
    pub metadata: Metadata,
}

impl AssetCollection {
    pub fn check(&self,world: &mut StateWorld, asset: &Asset) -> bool {
        let iri = asset.get_uid().clone();
        if iri.is_none() {
            return false;
        }
        let iri = iri.unwrap().as_str();
        let source = self.source.as_ref().unwrap().as_str();

        //check refinement
        let refinement = self.get_refinement();
        let mut refined = true;
        if let Some(refinement) = refinement {
            for constraint in refinement {
                match constraint {
                    ConstraintUnion::Constraint(constraint) => {
                        let mut world = world.clone();
                        let ret = constraint.eval(&mut world);
                        match ret {
                            Ok(false) => {
                                refined = false;
                            }
                            _ => {
                            }
                        }
                    }
                    ConstraintUnion::LogicConstraint(ac) => {
                        let mut world = world.clone();
                        let ret = constraint.eval(&mut world);
                        match ret {
                            Ok(false) => {
                                refined = false;
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
        }

        iri == source && refined
    }
}

//http://www.w3.org/ns/odrl/2/Asset
#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Asset {
    //unique identifier of the asset
    pub uid: Option<IriBuf>,
    //part of the asset collection
    pub partOf: Option<Vec<IriBuf>>,
    //refer to policy definition by IRI of Policy
    pub hasPolicy: Option<IriBuf>,
    //common metadata
    pub metadata: Option<Metadata>,
}

impl Asset {
    pub fn check(&self, world: &mut StateWorld, asset: &Asset) -> bool {
        let iri = asset.get_uid().clone();
        if iri.is_none() {
            return false;
        }

        let iri = iri.unwrap().as_str();
        let target = self.get_uid().clone().unwrap().as_str();

        //check has policy
        let hasPolicy = self.get_hasPolicy();
        if let Some(hasPolicy) = hasPolicy {
            let hasPolicy = hasPolicy.to_string();
            let policy = world.get_policy(hasPolicy);
            if let Some(policy) = policy {
                return policy.check(world, asset);
            }
        }

        if iri == target {
            return true;
        }

        //check partOf
        let mut part_refined = false;
        let partOf = self.get_partOf();
        if let Some(partOf) = partOf {
            for part in partOf {
                let part = part.to_string();
                //query partOf asset
                let ac = world.get_assets(part);
                if let Some(ac) = ac {
                    if !ac.check(world, asset) {
                        return false;
                    }
                }
            }
        }

        false
    }
}
