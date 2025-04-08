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
use reqwest::Proxy;
use serde_json::Value;
use static_iref::iri;
use crate::config;
use crate::linkdata::jsonld::{JsonLdAction, JsonLdAnyValue, JsonLdAsset, JsonLdConstraint, JsonLdConstraintOne, JsonLdDuty, JsonLdLogicConstraint, JsonLdOptionArray, JsonLdParser, JsonLdParty, JsonLdPermission, JsonLdPolicy, JsonLdProhibition};
use crate::model::constraint::Constraint;
use crate::model::constraint::LogicConstraint;
use crate::model::{constraint::ConstraintUnion};
use crate::model::action::{Action, ActionType};
use crate::model::asset::{Asset, AssetCollection, AssetUnion};
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint_left_operand::ConstraintLeftOperand;
use crate::model::constraint_left_operand::ConstraintLeftOperand::meteredTime;
use crate::model::constraint_operator::{ConstraintLogicOperator, ConstraintOperator};
use crate::model::constraint_right_operand::{ConstraintRightOperand, RightOperandReference, RightOperandType};
use crate::model::duty::Duty;
use crate::model::model_factory::ModelFactory;
use crate::model::party::{Party, PartyCollection, PartyUnion};
use crate::model::permission::Permission;
use crate::model::policy::{Agreement, PolicyUnion};
use crate::model::prohibition::Prohibition;
use crate::model::rule::Rule;
use crate::model::stateworld::{StateWorld, GLOBAL_WORLD_CACHE};

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

            let reference = RightOperandReference::builder()
                .reference(IriBuf::new(str).ok()).build();
            operand.set_reference(Some(reference));
        },
        _ => {
            return Err(anyhow!("Unsupported right operand type {} at {} {}",val,file!(),line!()));
        }
    }
    Ok(operand)
}

fn compile_constraint_one(json: &JsonLdConstraint) -> Result<Constraint,anyhow::Error> {
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
        constraint.set_status(Some(status));
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
        let left_operand_iri = left_operand.get_uid().as_str();
        if left_operand_iri.contains("timeInterval") || left_operand_iri.contains("timeWindow")  {
            //adjust operator to gt
            println!("Adjust operator to gt for timeInterval {}", left_operand_iri);
            constraint.set_operator(Some(ConstraintOperator::gt));
        }

        let left_operand = ConstraintLeftOperand::try_from(left_operand_iri)?;
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

    Ok(constraint)
}

fn compile_logic_constraint_one(json: &JsonLdLogicConstraint) -> Result<LogicConstraint,anyhow::Error> {
    let mut logic_constraint = LogicConstraint::default();
    //check uid
    if json.get_uid().is_some() {
        logic_constraint.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
    }

    //check operator
    if json.get_operator().is_some() {
        let operator = json.get_operator().clone().unwrap();
        let op = ConstraintLogicOperator::try_from(operator.get_uid().clone().unwrap().as_str()).ok();
        logic_constraint.set_operator(op);
    }

    if let Some(operands) = json.get_constraint() {
       match operands {
           JsonLdOptionArray::Single(constraint) => {
               let constraint = compile_constraint_one(constraint)?;
               logic_constraint.set_operand(Some(vec![constraint]));
           }
           JsonLdOptionArray::Multiple(constraints) => {
               let mut list = vec![];
               for constraint in constraints {
                   let constraint = compile_constraint_one(constraint)?;
                   list.push(constraint);
               }
               logic_constraint.set_operand(Some(list));
           }
       }
    }
    Ok(logic_constraint)
}

fn compile_constraint(json: &JsonLdOptionArray<JsonLdConstraintOne>) -> Result<Vec<ConstraintUnion>,anyhow::Error> {
    let mut constraints_parsed:Vec<ConstraintUnion> = vec![];
    match json {
        JsonLdOptionArray::Single(constraint) => {
            match constraint {
                JsonLdConstraintOne::Constraint(json) => {
                    let constraint = compile_constraint_one(json)?;
                    constraints_parsed.push(ConstraintUnion::Constraint(constraint));
                },
                JsonLdConstraintOne::LogicConstraint(json) => {
                    let logic_constraint = compile_logic_constraint_one(json)?;
                    constraints_parsed.push(ConstraintUnion::LogicConstraint(logic_constraint));
                }
            }
        }
        JsonLdOptionArray::Multiple(constraints) => {
            for constraint in constraints {
                match constraint {
                    JsonLdConstraintOne::Constraint(json) => {
                        let constraint = compile_constraint_one(json)?;
                        constraints_parsed.push(ConstraintUnion::Constraint(constraint));
                    },
                    JsonLdConstraintOne::LogicConstraint(json) => {
                        let logic_constraint = compile_logic_constraint_one(json)?;
                        constraints_parsed.push(ConstraintUnion::LogicConstraint(logic_constraint));
                    }
                }
            }
        }
    }
    Ok(constraints_parsed)
}

fn compile_party(json: &JsonLdParty) -> Option<PartyUnion> {
    //check party type
    let party_type = json.get_party_type().clone();
    let is_collection = party_type.is_some() && party_type.unwrap().contains("PartyCollection");
    let party = if is_collection {
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
                //check part of
                if json.get_part_of().is_some() {
                    let part_of = json.get_part_of().clone().unwrap();
                    let part = to_iri(part_of.get_uid().clone().unwrap().as_str()).unwrap();

                    party.set_partOf(vec![part]);
                }

                //check refinement
                if json.get_refinement().is_some() {
                    let refinement = json.get_refinement().clone().unwrap();
                    party.set_refinement(compile_constraint(&refinement).ok());
                }

                //Update state world
                // let mut world_cache = GLOBAL_WORLD_CACHE.;
                {
                    //TODO: fixme
                    // let world = world_cache.find_world("");
                    // if world.is_some() {
                    //     let world = world.unwrap();
                    //     world.add_party(party.get_uid().clone().unwrap().as_str(),party.clone());
                    // }
                }

                return Some(PartyUnion::Party(party));
            }
            return None;
        }
    }
}

fn compile_asset_one(json: &JsonLdAsset) -> Option<AssetUnion> {
    //check party type
    let asset_type = json.get_asset_type().clone();

    let is_collection = asset_type.is_some() && asset_type.unwrap().contains("Collection");
    let asset = if is_collection {
        AssetUnion::AssetCollection(AssetCollection::default())
    } else {
        AssetUnion::Asset(Asset::default())
    };

    match asset {
        AssetUnion::AssetCollection(mut ac) => {
            if json.get_source().is_some() {
                let source = json.get_source().clone().unwrap();
                ac.set_source(to_iri(source.as_str()));
                //check refinement
                if json.get_refinement().is_some() {
                    let refinement = json.get_refinement().clone().unwrap();
                    ac.set_refinement(compile_constraint(&refinement).ok());
                }

                return Some(AssetUnion::AssetCollection(ac));
            }
            return None;
        }
        AssetUnion::Asset(mut asset) => {
            let uid = json.get_uid().clone();
            asset.set_uid(to_iri(uid.as_str()));

            //check part of
            if json.get_part_of().is_some() {
                let part_of = json.get_part_of().clone().unwrap();
                asset.set_partOf(Some(vec![to_iri(part_of.as_str()).unwrap()]));
            }

            return Some(AssetUnion::Asset(asset));
        }
    }
}

fn compile_asset(json: &JsonLdOptionArray<JsonLdAsset>) -> Option<AssetUnion> {
    match json {
        JsonLdOptionArray::Single(asset) => {
            let asset = compile_asset_one(&asset);
            return asset;
        }
        JsonLdOptionArray::Multiple(assets) => {
            for asset in assets {
                let asset = compile_asset_one(&asset);
                return asset;
            }
        }
    }
    return None;
}

fn compile_profile(json: &JsonLdOptionArray<JsonLdAnyValue>) -> Option<Vec<IriBuf>> {
    let mut profiles:Vec<IriBuf> = vec![];
    match json {
        JsonLdOptionArray::Single(profile) => {
            let profile = profile.get_uid().clone().unwrap();
            let profile = to_iri(profile.as_str()).unwrap();
            profiles.push(profile);
        }
        JsonLdOptionArray::Multiple(iris) => {
            for profile in iris {
                let profile = profile.get_uid().clone().unwrap();
                let profile = to_iri(profile.as_str()).unwrap();
                profiles.push(profile);
            }
        }
    }
    return Some(profiles);
}

fn compile_conflict(json: JsonLdAnyValue) -> Option<ConflictStrategy> {
    let conflict = json.get_uid().clone().unwrap();
    let conflict = ConflictStrategy::try_from(conflict.as_str()).unwrap();
    return Some(conflict);
}

fn compile_action(json: JsonLdAction) -> Option<Action> {
    let mut action = Action::default();

    //check action type
    let action_id = json.get_uid().clone();
    let typ = ActionType::try_from(action_id.as_str()).unwrap();
    action.set_actionType(typ);

    //check included in
    if json.get_included_in().is_some() {
        let included_in = json.get_included_in().clone().unwrap();
        let mut actions = vec![];
        for act_id in included_in {
            let mut a = Action::default();
            let typ = ActionType::try_from(act_id.as_str()).unwrap();
            a.set_actionType(typ);
            actions.push(a);
        }
        action.set_includedIn(Some(actions));
    }

    //check implicits
    if json.get_implies().is_some() {
        let implies = json.get_implies().clone().unwrap();
        let mut actions = vec![];
        for act in implies {
            let mut a = Action::default();
            let action_id = act.get_uid().as_str();
            let typ = ActionType::try_from(action_id).unwrap();
            a.set_actionType(typ);
            actions.push(a);
        }
        action.set_implies(Some(actions));
    }

    //check refinements
    if json.get_refinement().is_some() {
        let refinement = json.get_refinement().clone().unwrap();
        action.set_refinements(compile_constraint(&refinement).ok());
    }

    return Some(action);
}

fn compile_permission_one(json: &JsonLdPermission) -> Result<Rule,anyhow::Error> {
    let mut rule = Rule::default();

    //Optional uid check
    if json.get_uid().is_some() {
        rule.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
    }

    //check assigner
    if json.get_assigner().is_some() {
        let assigner = json.get_assigner().clone().unwrap();
        let assigner = compile_party(&assigner);
        if assigner.is_some() {
            rule.set_assigner(assigner);
        }
    }

    //check assignee
    if json.get_assignee().is_some() {
        let assignee = json.get_assignee().clone().unwrap();
        let assignee = compile_party(&assignee);
        if assignee.is_some() {
            rule.set_assignee(assignee);
        }
    }

    //check action
    if json.get_action().is_some() {
        let action = json.get_action().clone().unwrap();
        let action = compile_action(action);
        if action.is_some() {
            rule.set_action(Some(action.unwrap()));
        }
    }

    //check target
    if json.get_target().is_some() {
        let target = json.get_target().clone().unwrap();
        let target = compile_asset(&target);
        if target.is_some() {
            rule.set_target(Some(target.unwrap()));
        }
    }

    //check constraint
    if json.get_constraint().is_some() {
        let constraint = json.get_constraint().clone().unwrap();
        rule.set_constraint(compile_constraint(&constraint).ok());
    }

    return Ok(rule);
}

fn compile_duty_one(json: &JsonLdDuty) -> Result<Rule,anyhow::Error> {
    let mut rule = Rule::default();

    //Optional uid check
    if json.get_uid().is_some() {
        rule.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
    }

    //check assigner
    if json.get_assigner().is_some() {
        let assigner = json.get_assigner().clone().unwrap();
        let assigner = compile_party(&assigner);
        if assigner.is_some() {
            rule.set_assigner(assigner);
        }
    }

    //check assignee
    if json.get_assignee().is_some() {
        let assignee = json.get_assignee().clone().unwrap();
        let assignee = compile_party(&assignee);
        if assignee.is_some() {
            rule.set_assignee(assignee);
        }
    }

    //check action
    if json.get_action().is_some() {
        let action = json.get_action().clone().unwrap();
        let action = compile_action(action);
        if action.is_some() {
            rule.set_action(Some(action.unwrap()));
        }
    }

    //check target
    if json.get_target().is_some() {
        let target = json.get_target().clone().unwrap();
        let target = compile_asset(&target);
        if target.is_some() {
            rule.set_target(Some(target.unwrap()));
        }
    }

    //check constraint
    if json.get_constraint().is_some() {
        let constraint = json.get_constraint().clone().unwrap();
        rule.set_constraint(compile_constraint(&constraint).ok());
    }

    return Ok(rule);
}

fn compile_prohibition_one(json: &JsonLdProhibition) -> Result<Rule,anyhow::Error> {
    let mut rule = Rule::default();

    //Optional uid check
    if json.get_uid().is_some() {
        rule.set_uid(to_iri(json.get_uid().clone().unwrap().as_str()));
    }

    //check assigner
    if json.get_assigner().is_some() {
        let assigner = json.get_assigner().clone().unwrap();
        let assigner = compile_party(&assigner);
        if assigner.is_some() {
            rule.set_assigner(assigner);
        }
    }

    //check assignee
    if json.get_assignee().is_some() {
        let assignee = json.get_assignee().clone().unwrap();
        let assignee = compile_party(&assignee);
        if assignee.is_some() {
            rule.set_assignee(assignee);
        }
    }

    //check action
    if json.get_action().is_some() {
        let action = json.get_action().clone().unwrap();
        let action = compile_action(action);
        if action.is_some() {
            rule.set_action(Some(action.unwrap()));
        }
    }

    //check target
    if json.get_target().is_some() {
        let target = json.get_target().clone().unwrap();
        let target = compile_asset(&target);
        if target.is_some() {
            rule.set_target(Some(target.unwrap()));
        }
    }

    //check constraint
    if json.get_constraint().is_some() {
        let constraint = json.get_constraint().clone().unwrap();
        rule.set_constraint(compile_constraint(&constraint).ok());
    }

    return Ok(rule);
}

fn compile_permission(json: &JsonLdOptionArray<JsonLdPermission>) -> Result<Vec<Permission>,anyhow::Error> {
    let mut permissions_parsed: Vec<Permission> = vec![];

    return match json {
        JsonLdOptionArray::Single(permission) => {
            let duty = compile_permission_one(permission)?;
            let perm = Permission::builder().duty(duty).build();
            permissions_parsed.push(perm);
            Ok(permissions_parsed)
        }
        JsonLdOptionArray::Multiple(permissions) => {
            for perm in permissions {
                let duty = compile_permission_one(perm)?;
                let perm = Permission::builder().duty(duty).build();
                permissions_parsed.push(perm);
            }
            Ok(permissions_parsed)
        }
    }
}

fn compile_obligation(json: &JsonLdOptionArray<JsonLdDuty>) -> Result<Vec<Duty>,anyhow::Error> {
    let mut obligations_parsed: Vec<Duty> = vec![];

    return match json {
        JsonLdOptionArray::Single(obligation) => {
            let duty = compile_duty_one(obligation)?;
            let duty = Duty::builder().rule(duty).build();
            obligations_parsed.push(duty);
            Ok(obligations_parsed)
        }
        JsonLdOptionArray::Multiple(obligations) => {
            for obligation in obligations {
                let duty = compile_duty_one(obligation)?;
                let duty = Duty::builder().rule(duty).build();
                obligations_parsed.push(duty);
            }
            Ok(obligations_parsed)
        }
    }
}

fn compile_prohibition(json: &JsonLdOptionArray<JsonLdProhibition>) -> Result<Vec<Prohibition>,anyhow::Error> {
    let mut prohibitions_parsed: Vec<Prohibition> = vec![];

    return match json {
        JsonLdOptionArray::Single(prohibition) => {
            let rule = compile_prohibition_one(prohibition)?;
            let prohibit = Prohibition::builder().rule(rule).build();
            prohibitions_parsed.push(prohibit);
            Ok(prohibitions_parsed)
        }
        JsonLdOptionArray::Multiple(prohibitions) => {
            for prohibition in prohibitions {
                let rule = compile_prohibition_one(prohibition)?;
                let prohibit = Prohibition::builder().rule(rule).build();
                prohibitions_parsed.push(prohibit);
            }
            Ok(prohibitions_parsed)
        }
    }
}

fn  compile_inherit_one(json: &JsonLdAnyValue) -> Result<IriBuf,anyhow::Error> {
    let inherit = json.get_uid().clone().unwrap();
    let inherit = to_iri(inherit.as_str()).unwrap();
    Ok(inherit)
}

fn compile_inherit_from(json: &JsonLdOptionArray<JsonLdAnyValue>) -> Result<Vec<IriBuf>,anyhow::Error> {
    let mut inherit_from: Vec<IriBuf> = vec![];
    return match json {
        JsonLdOptionArray::Single(inherit) => {
            let inh = compile_inherit_one(inherit)?;
            inherit_from.push(inh);
            Ok(inherit_from)
        }
        JsonLdOptionArray::Multiple(inherits) => {
            for inherit in inherits {
                let inh = compile_inherit_one(inherit)?;
                inherit_from.push(inh);
            }
            Ok(inherit_from)
        }
    }
}

impl OdrlLoader {
    pub async  fn load_file(iri: String, path: String) -> Result<ExpandedDocument, anyhow::Error> {
        // let proxy = Proxy::https("http://127.0.0.1:9981");
        // let mut parse = JsonLdParser::new(proxy.ok());
        let mut parse = JsonLdParser::new(None);
        let val = std::fs::read_to_string(path)?;

        let document = parse.parse(iri, val.to_string()).await;
        document
    }

    pub async  fn load_json(iri: String, json: String) -> Result<ExpandedDocument, anyhow::Error> {
        let mut parse = JsonLdParser::new(None);
        let document = parse.parse(iri, json).await;
        document
    }

    async fn normalize(policy: &mut JsonLdPolicy) -> Result<(), anyhow::Error> {
        //get policy level assigner
        let policy_assigner = policy.get_assigner().clone();
        //get policy level assignee
        let policy_assignee = policy.get_assignee().clone();
        //get policy level asset
        let policy_asset = policy.get_target().clone();

        //get policy level action
        let policy_action = policy.get_action().clone();

        //Normalize permissions
        let permissions = policy.get_permission_mut();
        if let Some(permission) = permissions {
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

                   //check permission level action
                   let action = permission.get_action();
                   if action.is_none() && policy_action.is_some() {
                        permission.set_action(policy_action.clone());
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

                        let action = permission.get_action();
                        if action.is_none() && policy_action.is_some() {
                            permission.set_action(policy_action.clone());
                        }
                    }
                }
            }
        }

        //Normalize obligations
        let obligations = policy.get_obligation_mut();
        if let Some(obligation) =  obligations {
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
        if let Some(prohibition) = prohibitions {
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

                {
                    let config = config::CONFIG.lock().unwrap();
                    if config.verbose {
                        println!(">>--------------------Pretty printed JSON-LD Expanded Document-----------------------------<<");
                        println!("{}", pretty);
                        println!(">>--------------------Pretty printed JSON-LD end-----------------------------<<");
                    }
                }

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

                {
                    let config = config::CONFIG.lock().unwrap();
                    if config.verbose {
                        println!(">>--------------------ODRL AST TOKEN TREE-----------------------------<<");
                        println!("{:#?}",policy);
                        println!(">>--------------------ODRL AST TOKEN end-----------------------------<<");
                    }
                }

                Ok(policy)
            }
            Err(err) => {
                Err(anyhow!("Error during compaction: {err:#?}"))
            }
        }
    }

    pub async fn compile(policy: &JsonLdPolicy) -> Result<PolicyUnion, anyhow::Error> {
        let mut type_ = policy.get_policy_type().clone();
        if type_.is_none() {
            type_ = Some("http://www.w3.org/ns/odrl/2/Set".to_string());
        }

        let evalator = ModelFactory::create(type_.unwrap());
        match evalator {
            crate::model::policy::PolicyUnion::Agreement(mut eval) => {
                let eval = eval.get_policy_mut();

                //copy policy uid to eval
                let uid = policy.get_uid().clone();
                eval.set_uid(IriBuf::new(uid.clone()).ok());

                //initialize world
                let world = StateWorld::builder().uid(IriBuf::new(uid.clone()).ok()).build();
                {
                    //cache world
                    let world_cache = GLOBAL_WORLD_CACHE.clone();
                    world_cache.add_world(uid.as_str(),world);
                }

                //check and copy assignee
                let assignee = policy.get_assignee().clone();
                if assignee.is_some() {
                    eval.set_assignee(compile_party(&assignee.unwrap()));
                }

                //check and copy assigner
                let assigner = policy.get_assigner().clone();
                if assigner.is_some() {
                    eval.set_assigner(compile_party(&assigner.unwrap()));
                }

                //check and copy target
                let target = policy.get_target().clone();
                if target.is_some() {
                    eval.set_target(compile_asset(&target.unwrap()));
                }

                //check and copy profile
                let profile = policy.get_profile().clone();
                eval.set_profile(compile_profile(&profile));

                //check and copy conflict
                let conflict = policy.get_conflict().clone().unwrap();
                eval.set_conflict(compile_conflict(conflict));

                //check and copy action
                let action = policy.get_action().clone();
                if action.is_some() {
                    let action = action.unwrap();
                    eval.set_action(compile_action(action));
                }

                //check and copy constraint
                let constraint = policy.get_constraint().clone();
                if constraint.is_some() {
                    let constraint = constraint.unwrap();
                    eval.set_constraint(compile_constraint(&constraint).ok());
                }

                //check and copy permission
                let permission = policy.get_permission().clone();
                if permission.is_some() {
                    let permission = permission.unwrap();
                    eval.set_permission(compile_permission(&permission).ok());
                }

                //check and copy obligation
                let obligation = policy.get_obligation().clone();
                if obligation.is_some() {
                    let obligation = obligation.unwrap();
                    eval.set_obligation(compile_obligation(&obligation).ok());
                }

                //check and copy prohibition
                let prohibition = policy.get_prohibition().clone();
                if prohibition.is_some() {
                    let prohibition = prohibition.unwrap();
                    eval.set_prohibition(compile_prohibition(&prohibition).ok());
                }

                // check and copy inheritFrom
                let inherit_from = policy.get_inherit_from().clone();
                if inherit_from.is_some() {
                    let inherit_from = inherit_from.unwrap();
                    eval.set_inheritFrom(compile_inherit_from(&inherit_from).ok());
                }

                let agreement = Agreement::builder().policy(eval.clone()).build();
                return Ok(PolicyUnion::Agreement(agreement));
            }
            _ => {}
        }

        Err(anyhow!("Error during compilation"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::config::Config;
    use super::*;
    #[tokio::test]
    pub async fn test() {
        use std::borrow::BorrowMut;
        let path = "src/data/sample.jsonld";

        {
            let mut config = config::CONFIG.lock().unwrap();
            let config = config.borrow_mut();
            config.set_verbose(true);
        }

        let doc = OdrlLoader::load_file("http://www.w3.org/ns/odrl/2".to_string(), path.to_string());
        let doc = doc.await;
        let expanded = doc.unwrap();

        let policy = OdrlLoader::parse(expanded).await;

        let policy = OdrlLoader::compile(&mut policy.unwrap()).await;
    }
}