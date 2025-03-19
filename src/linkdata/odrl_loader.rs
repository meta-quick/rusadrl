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

#![warn(unused_variables)]
#![allow(dead_code)]
#![warn(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use anyhow::anyhow;
use iref::IriBuf;
use json_ld::Compact;
use json_ld::context_processing::ProcessedRef;
use json_ld::object::Any;
use json_ld_core::{ExpandedDocument};
use json_ld_core::Node;
use serde_json::Value;
use static_iref::iri;
use crate::linkdata::jsonld::{JsonLdConstraintOne, JsonLdOptionArray, JsonLdParser, JsonLdParty, JsonLdPolicy};
use crate::model::constraint::Constraint;
use crate::model::constraint::LogicConstraint;
use crate::model::{constraint::ConstraintUnion};
use crate::model::constraint_left_operand::ConstraintLeftOperand;
use crate::model::constraint_left_operand::ConstraintLeftOperand::meteredTime;
use crate::model::constraint_operator::ConstraintOperator;
use crate::model::constraint_right_operand::{ConstraintRightOperand, RightOperandReference, RightOperandType};
use crate::model::model_factory::ModelFactory;
use crate::model::party::{Party, PartyCollection, PartyUnion};

#[derive(Debug)]
pub struct OdrlLoader;

fn to_iri(iri: &str) -> Option<IriBuf> {
    Some(IriBuf::new(iri.to_string()).unwrap())
}

fn to_right_operand(val: serde_json::Value) -> Result<ConstraintRightOperand,anyhow::Error> {
    let mut operand = ConstraintRightOperand::default();
    match val {
        serde_json::Value::String(str) => {
            operand.set_ty(RightOperandType::Literal);
            operand.set_value(Some(str));
        },
        serde_json::Value::Number(num) => {
            operand.set_ty(RightOperandType::Literal);
            operand.set_value(Some(num.to_string()));
        }
        serde_json::Value::Bool(bool) => {
            operand.set_ty(RightOperandType::Literal);
            operand.set_value(Some(bool.to_string()));
        }
        serde_json::Value::Array(arr) => {
            operand.set_ty(RightOperandType::LiteralSet);
            let mut set:Vec<String> = vec![];
            for item in arr {
                match item {
                    serde_json::Value::String(str) => {
                        set.push(str);
                    }
                    serde_json::Value::Number(num) => {
                        set.push(num.to_string());
                    }
                    _ => {
                    }
                }
            }
            operand.set_values(Some(set));
        }
        _ => {
            return Err(anyhow!("Unsupported right operand type {} at {} {}",val,file!(),line!()));
        }
    }
    Ok(operand)
}

fn to_right_operand_reference(val: serde_json::Value) -> Result<ConstraintRightOperand,anyhow::Error> {
    let mut operand = ConstraintRightOperand::default();
    match val {
        serde_json::Value::String(str) => {
            operand.set_ty(RightOperandType::Reference);

            let mut reference = RightOperandReference::builder()
                .reference(IriBuf::new(str).ok()).build();
            operand.set_reference(Some(reference));
        },
        _ => {
            return Err(anyhow!("Unsupported right operand type {} at {} {}",val,file!(),line!()));
        }
    }
    Ok(operand)
}

fn compile_constraint(json: &JsonLdOptionArray<JsonLdConstraintOne>) -> Result<Vec<ConstraintUnion>,anyhow::Error> {
    let mut constraints:Vec<ConstraintUnion> = vec![];
    match json {
        JsonLdOptionArray::Single(constraint) => {
            match constraint {
                JsonLdConstraintOne::Constraint(json) => {
                    let mut constraint = Constraint::default();
                    if json.get_uid().is_some() {
                        constraint.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
                    }

                    if json.get_unit().is_some() {
                        constraint.set_unit(json.get_unit().clone().unwrap());
                    }

                    //check  data type
                    if json.get_data_type().is_some() {
                        let data_type = json.get_data_type().clone().unwrap();
                        constraint.set_dataType(data_type.get_value().clone().unwrap());
                    }

                    //check status
                    if json.get_status().is_some() {
                        let status = json.get_status().clone().unwrap();
                        constraint.set_status(status.eq(&"true"));
                    }

                    //check operator
                    if json.get_operator().is_some() {
                        let operator = json.get_operator().clone().unwrap();
                        let op = ConstraintOperator::try_from(operator.get_uid().as_str()).unwrap();
                        constraint.set_operator(Some(op));
                    } else {
                        return Err(anyhow!("None constraint operator found, error at {},{}",file!(),line!()));
                    }

                    //check left operand
                    if json.get_left_operand().is_some() {
                        let left_operand = json.get_left_operand().clone().unwrap();
                        let left_operand = ConstraintLeftOperand::try_from(left_operand.get_uid().as_str())?;
                        constraint.set_leftOperand(Some(left_operand));
                    } else {
                        return Err(anyhow!("None left operand found, error at {},{}",file!(),line!()));
                    }

                    //check right operand
                    if json.get_right_operand().is_some() {
                        let right_operand = json.get_right_operand().clone().unwrap();
                        constraint.set_rightOperand(to_right_operand(right_operand).ok());
                    } else if json.get_right_operand_reference().is_some() {
                        let right_operand_ref = json.get_right_operand_reference().clone().unwrap();
                        constraint.set_rightOperand(to_right_operand_reference(right_operand_ref).ok());
                    } else {
                        return Err(anyhow!("None right operand found, error at {},{}",file!(),line!()));
                    }
                },
                JsonLdConstraintOne::LogicConstraint(constraint) => {

                }
            }
        }
        JsonLdOptionArray::Multiple(constraints) => {
            for constraint in constraints {
                match constraint {
                    JsonLdConstraintOne::Constraint(constraint) => {
                    },
                    JsonLdConstraintOne::LogicConstraint(constraint) => {
                    }
                }
            }
        }
    }
    Ok(constraints)
}

fn compile_party(json: &JsonLdParty) -> Option<PartyUnion> {
    //check party type
    let mut party_type = json.get_party_type().clone();
    let is_collection = party_type.is_some() && party_type.unwrap().contains("PartyCollection");
    let mut party = if is_collection {
        PartyUnion::PartyCollection(PartyCollection::default())
    } else {
        PartyUnion::Party(Party::default())
    };

    match party {
        PartyUnion::PartyCollection(mut collection) => {
            if json.get_source().is_some() {
                let source = json.get_source().clone().unwrap();
                let source = to_iri(source.get_uid().clone().unwrap().as_str());
                collection.set_source(source);

                // handle refinement
                if json.get_refinement().is_some() {
                    let refinement = json.get_refinement().clone().unwrap();
                    collection.set_refinement(compile_constraint(&refinement).ok());
                }
                return Some(PartyUnion::PartyCollection(collection));
            }
            return None;
        }
        PartyUnion::Party(mut party) => {
            if json.get_uid().is_some()  {
                party.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
            }

            //check part of
            // if json.get_part_of().is_some() {
            //     let part_of = json.get_part_of().clone().unwrap();
            //     let parts = to_iri(part_of.get_uid().clone().unwrap().as_str());
            //     party.set_partOf(vec![parts.unwrap()]);
            // }
            return Some(PartyUnion::Party(party));
        }
    }
}

impl OdrlLoader {
    pub async  fn load(iri: String, path: String) -> Result<ExpandedDocument, anyhow::Error> {
        let mut parse = JsonLdParser::new(None);
        let val = std::fs::read_to_string(path)?;

        let document = parse.parse(iri, val.to_string()).await;
        document
    }

    async fn normalize(policy: &mut JsonLdPolicy) -> Result<(), anyhow::Error> {
        //get policy level assigner
        let policy_assigner = policy.get_assigner().clone();
        //get policy level assignee
        let policy_assignee = policy.get_assignee().clone();
        //get policy level asset
        let policy_asset = policy.get_target().clone();
        //Normalize permissions
        let permissions = policy.get_permission_mut();
        for permission in  permissions {
            match permission {
                JsonLdOptionArray::Single(permission) => {
                   //check permission level assigner
                   let assigner = permission.get_assigner();
                   if assigner.is_none() && policy_assigner.is_some() {
                       permission.set_assigner(policy_assigner.clone());
                   }

                   //check permission level assignee
                   let assignee = permission.get_assignee();
                   if assignee.is_none() && policy_assignee.is_some() {
                       permission.set_assignee(policy_assignee.clone());
                   }
                   //check permission level asset
                   let asset = permission.get_target();
                   if asset.is_none() && policy_asset.is_some() {
                       permission.set_target(policy_asset.clone());
                   }
                },
                JsonLdOptionArray::Multiple(permissions) => {
                    for permission in permissions {
                        //check permission level assigner
                        let assigner = permission.get_assigner();
                        if assigner.is_none() && policy_assigner.is_some() {
                            permission.set_assigner(policy_assigner.clone());
                        }

                        //check permission level assignee
                        let assignee = permission.get_assignee();
                        if assignee.is_none() && policy_assignee.is_some() {
                            permission.set_assignee(policy_assignee.clone());
                        }
                        //check permission level asset
                        let asset = permission.get_target();
                        if asset.is_none() && policy_asset.is_some() {
                            permission.set_target(policy_asset.clone());
                        }
                    }
                }
            }
        }

        //Normalize obligations
        let obligations = policy.get_obligation_mut();
        for obligation in  obligations {
            match obligation {
                JsonLdOptionArray::Single(obligation) => {
                    //check obligation level assigner
                    let assigner = obligation.get_assigner();
                    if assigner.is_none() && policy_assignee.is_some() {
                        obligation.set_assigner(policy_assignee.clone());
                    }

                    //check obligation level assignee
                    let assignee = obligation.get_assignee();
                    if assignee.is_none() && policy_assignee.is_some() {
                        obligation.set_assignee(policy_assignee.clone());
                    }

                    let target = obligation.get_target();
                    if target.is_none() && policy_asset.is_some() {
                        obligation.set_target(policy_asset.clone());
                    }
                },
                JsonLdOptionArray::Multiple(obligations) => {
                    for obligation in obligations {
                        //check obligation level assigner
                        let assigner = obligation.get_assigner();
                        if assigner.is_none() && policy_assignee.is_some() {
                            obligation.set_assigner(policy_assignee.clone());
                        }

                        //check obligation level assignee
                        let assignee = obligation.get_assignee();
                        if assignee.is_none() && policy_assignee.is_some() {
                            obligation.set_assignee(policy_assignee.clone());
                        }

                        let target = obligation.get_target();
                        if target.is_none() && policy_asset.is_some() {
                            obligation.set_target(policy_asset.clone());
                        }
                    }
                }
            }
        }

        //Normalize prohibitions
        let prohibitions = policy.get_prohibition_mut();
        for prohibition in prohibitions {
            match prohibition {
                JsonLdOptionArray::Single(prohibition) => {
                    //check prohibition level assigner
                    let assigner = prohibition.get_assigner();
                    if assigner.is_none() && policy_assignee.is_some() {
                        prohibition.set_assigner(policy_assignee.clone());
                    }
                    //check prohibition level assignee
                    let assignee = prohibition.get_assignee();
                    if assignee.is_none() && policy_assignee.is_some() {
                        prohibition.set_assignee(policy_assignee.clone());
                    }

                    let target = prohibition.get_target();
                    if target.is_none() && policy_asset.is_some() {
                        prohibition.set_target(policy_asset.clone());
                    }
                }
                JsonLdOptionArray::Multiple(prohibitions) => {
                    for prohibition in prohibitions {
                        //check prohibition level assigner
                        let assigner = prohibition.get_assigner();
                        if assigner.is_none() && policy_assignee.is_some() {
                            prohibition.set_assigner(policy_assignee.clone());
                        }
                        //check prohibition level assignee
                        let assignee = prohibition.get_assignee();
                        if assignee.is_none() && policy_assignee.is_some() {
                            prohibition.set_assignee(policy_assignee.clone());
                        }

                        let target = prohibition.get_target();
                        if target.is_none() && policy_asset.is_some() {
                            prohibition.set_target(policy_asset.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn parse(expanded: ExpandedDocument) -> Result<JsonLdPolicy, anyhow::Error> {
        let unprocessed = json_ld_syntax::context::Context::null();
        let processed = json_ld_core::context::Context::new(
            Some(IriBuf::new("http://example.org/".to_owned()).unwrap())
        );
        let context = ProcessedRef::new(&unprocessed, &processed);
        let result = expanded.compact(context, &mut json_ld::NoLoader::default()).await;

        match result {
            Ok(compacted) => {
                let json_string = compacted.to_string();
                let v: Value = serde_json::from_str(&json_string).unwrap();
                let pretty = serde_json::to_string_pretty(&v).unwrap();
                println!("{}", pretty);

                let policy =   serde_json::from_str::<JsonLdPolicy>(&json_string);
                let policy = match policy {
                    Ok(policy) => policy,
                    Err(err) => {
                        return Err(anyhow!("Error during compaction: {err:#?}"));
                    }
                };

                //Normalize
                let mut policy = policy;
                let _ = OdrlLoader::normalize(&mut policy).await;
                // println!("{:#?}",policy);
                Ok(policy)
            }
            Err(err) => {
                Err(anyhow!("Error during compaction: {err:#?}"))
            }
        }
    }

    pub async fn compile(policy: &JsonLdPolicy) -> Result<(), anyhow::Error> {
        let mut type_ = policy.get_policy_type().clone();
        if type_.is_none() {
            type_ = Some("http://www.w3.org/ns/odrl/2/Set".to_string());
        }

        let mut evalator = ModelFactory::create(type_.unwrap());
        match evalator {
            crate::model::policy::PolicyUnion::Agreement(mut eval) => {
                let eval = eval.get_policy_mut();

                //copy policy uid to eval
                let uid = policy.get_uid().clone();
                eval.set_uid(IriBuf::new(uid).ok());

                //check and copy assigner
                let assignee = policy.get_assignee().clone();
                if assignee.is_some() {
                    eval.set_assignee(compile_party(&assignee.unwrap()));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn test() {
        let path = "src/data/sample.jsonld";
        let doc = OdrlLoader::load("http://www.w3.org/ns/odrl/2".to_string(), path.to_string());
        let doc = doc.await;
        let expanded = doc.unwrap();

        let mut policy = OdrlLoader::parse(expanded).await;
        let a = OdrlLoader::compile(&mut policy.unwrap()).await;
        // println!("{:#?}", a);
    }
}