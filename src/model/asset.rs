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

#[derive(Debug,Clone)]
pub enum AssetUnion {
    Asset(Asset),
    AssetCollection(AssetCollection),
}

pub struct AssetInferencer;
impl AssetInferencer {
    pub fn infer(world: &mut StateWorld, asset: AssetUnion,candidate: Asset) -> Result<bool, anyhow::Error>{
        match asset {
            AssetUnion::Asset(asset) => {
                //Only need to check uid and partOf
                let asset_uid = asset.get_uid();
                if let None = asset_uid {
                    return Ok(false);
                }
                let asset_uid = asset_uid.clone().unwrap();
                let asset_uid = asset_uid.as_str();

                let candidate_uid = candidate.get_uid();
                if let None = candidate_uid {
                    return Ok(false);
                }
                let candidate_uid = candidate_uid.clone().unwrap();
                let candidate_uid = candidate_uid.as_str();

                if asset_uid == candidate_uid {
                    return Ok(true);
                }

                /*
                 * candidate is a sub asset of asset, basic logic is
                 * candidate.partOf.contains(asset.uid)
                 * partOf is not supported yet
                 */
                let partOf = candidate.get_partOf().clone();
                if let None = partOf {
                    return Ok(false);
                }
                let partOf = partOf.unwrap();
                for part in partOf {
                    let part = part.as_str();
                    if asset_uid == part {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
            AssetUnion::AssetCollection(collection) => {
                let collection_uid = collection.get_source();
                if let None = collection_uid {
                    return Ok(false);
                }
                let collection_uid = collection_uid.clone().unwrap();
                let collection_uid = collection_uid.as_str();

                let candidate_uid = candidate.get_uid();
                if let None = candidate_uid {
                    return Ok(false);
                }
                let candidate_uid = candidate_uid.clone().unwrap();
                let candidate_uid = candidate_uid.as_str();

                let mut find_partOf = false;
                if collection_uid != candidate_uid {
                    //check partOf
                    let partOf = candidate.get_partOf().clone();
                    if let None = partOf {
                        return Ok(false);
                    }
                    let partOf = partOf.unwrap();
                    for part in partOf {
                        let part = part.as_str();
                        if collection_uid == part {
                            find_partOf = true;
                            break;
                        }
                    }
                }

                if collection_uid == candidate_uid || find_partOf {
                    //check refinement
                    let refinement = collection.get_refinement();
                    if let Some(refinement) = refinement {
                        let mut refined = true;
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
                                    let ret = ac.eval(&mut world);
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
                        return Ok(refined);
                    }
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}