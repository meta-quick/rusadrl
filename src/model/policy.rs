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
#![allow(non_camel_case_types)]

use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::action::{Action, ActionInferencer, ActionType};
use crate::model::asset::{Asset, AssetInferencer, AssetUnion};
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint::{ConstraintInference, ConstraintUnion};
use crate::model::duty::Duty;
use crate::model::metadata::Metadata;
use crate::model::eval::Evaluator;
use crate::model::party::{Party, PartyInferencer, PartyUnion};
use crate::model::permission::Permission;
use crate::model::prohibition::Prohibition;
use crate::model::stateworld::StateWorld;

//Identifier:	http://www.w3.org/ns/odrl/2/Policy
#[derive(Debug,Default,Builder,Setter,Getter,GetterMut,Clone)]
pub struct Policy {
    //Policy must have a unique identifier
    pub uid: Option<IriBuf>,
    pub profile: Option<Vec<IriBuf>>,

    pub action: Option<Action>,
    pub assignee: Option<PartyUnion>,
    pub assigner: Option<PartyUnion>,
    pub conflict: Option<ConflictStrategy>,
    pub permission: Option<Vec<Permission>>,
    pub prohibition: Option<Vec<Prohibition>>,
    pub obligation: Option<Vec<Duty>>,
    pub target: Option<AssetUnion>,
    pub inheritFrom : Option<Vec<IriBuf>>,
    pub constraint: Option<Vec<ConstraintUnion>>,
    pub relation: Option<Vec<IriBuf>>,
    pub function: Option<Vec<IriBuf>>,

    pub metadata: Option<Metadata>,
}

impl Policy {
    pub fn new() -> Self {
       Self::default()
    }
}

//http://www.w3.org/ns/odrl/2/Agreement
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Agreement {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Offer
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Offer {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Set
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Set {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Privacy
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Privacy {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Request
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Request {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Assertion
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Assert {
    pub policy: Policy,
}

//http://www.w3.org/ns/odrl/2/Ticket
#[derive(Debug,Default,Builder,Getter,GetterMut, Clone)]
pub struct Ticket {
    pub policy: Policy,
}

#[derive(Debug, Clone)]
pub enum PolicyUnion {
    Agreement(Agreement),
    Offer(Offer),
    Set(Set),
    Privacy(Privacy),
    Request(Request),
    Assert(Assert),
    Ticket(Ticket),
}

#[derive(Debug,Default,Builder,Setter,GetterMut,Clone)]
pub struct OdrlRequest{
    pub action: Option<IriBuf>,
    pub assignee: Option<IriBuf>,
    pub assigner: Option<IriBuf>,
    pub target: Option<IriBuf>,
}

impl OdrlRequest {
    pub fn get_assignee(&self) -> Option<Party> {
        if self.assignee.is_some() {
            Some(Party::builder().uid(self.assignee.clone()).build())
        } else {
            None
        }
    }
    pub fn get_assigner(&self) -> Option<Party> {
        if self.assigner.is_some() {
            Some(Party::builder().uid(self.assigner.clone()).build())
        } else {
            None
        }
    }

    pub fn get_action(&self) -> Option<Action> {
        if self.action.is_some() {
            let ty = self.action.clone().unwrap();
            let ty = ty.as_str();
            let action = Action::builder().actionType(ActionType::try_from(ty).unwrap()).build();
            Some(action)
        }else {
            None
        }
    }

    pub fn get_target(&self) -> Option<Asset> {
        if self.target.is_some() {
            Some(Asset::builder().uid(self.target.clone()).build())
        }else {
            None
        }
    }
}

impl Evaluator for Agreement  {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
       let policy = &self.policy;
       let mut conflict = policy.get_conflict().clone();
       if conflict.is_none() {
           conflict = Some(ConflictStrategy::perm);
       }
       let conflict = conflict.unwrap();

       let candidate = req.get_action().clone();
       if candidate.is_none() {
           return Ok(false);
       }

       //check allow permissions
       let permissions = policy.get_permission();
       if permissions.is_none() {
          return Ok(false);
       }
       let prohibitions = policy.get_prohibition();

       /*
        * 1. Check Assigner and Assignee match in each permission
        * 2. Check Target match in each permission
        * 3. Check Action match in each permission
        * 4. Check Constraint match in each permission
        * 5. Check Obligation match in each permission
        * 6. Check Above match in each prohibition
        * 7. Check Conflict Strategy
        */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                let candidate_assignee = candidate_assignee.clone();
                let policy_assignee = permission.get_assignee().clone();
                let mut assignee_verified = false;
                if candidate_assignee.is_some() && policy_assignee.is_some() {
                    let union = policy_assignee.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                    if let Ok(true) = ret {
                        assignee_verified = true;
                    }
                }
                if !assignee_verified {
                    //no need to check other parts, must have exact one assignee
                    continue;
                }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = assigner.clone().unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                } else {
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

       //check prohibition
       if permitted {
           let mut prohibited = false;
           if let Some(prohibits) = prohibitions {
               for prohibit in prohibits {
                   let candidate_assignee = candidate_assignee.clone();
                   let policy_assignee = prohibit.get_assignee().clone();

                   //do assignee verification
                   let mut assignee_verified = false;
                   if candidate_assignee.is_some() && policy_assignee.is_some() {
                       let union = policy_assignee.unwrap();
                       let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                       if let Ok(true) = ret {
                           assignee_verified = true;
                       }
                   }
                   if !assignee_verified {
                       //no need to check other parts, must have exact one assignee
                       continue;
                   }

                   //do assigner verification
                   let candidate_assigner = candidate_assigner.clone();
                   let policy_assigner = prohibit.get_assigner().clone();
                   let mut assigner_verified = false;
                   let assigner = prohibit.get_assigner();
                   if candidate_assigner.is_some() && assigner.is_some() {
                       let union = policy_assigner.unwrap();
                       let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                       if let Ok(true) = ret {
                           assigner_verified = true;
                       }
                   }
                   if !assigner_verified {
                       //no need to check other parts, must have exact one assigner
                       continue;
                   }

                   //do action verification
                   let candidate_action = candidate_action.clone().unwrap();
                   let policy_action = prohibit.get_action().clone().unwrap();
                   let result = ActionInferencer::infer(world,policy_action,candidate_action);
                   let mut action_verified = false;
                   if let Ok(true) = result {
                       action_verified = true;
                   }

                   if !action_verified {
                       //no need to check other parts, must have exact one action
                       continue;
                   }

                   //do target verification
                   let candidate_target = req.get_target().clone().unwrap();
                   let policy_target = prohibit.get_target().clone().unwrap();
                   let result = AssetInferencer::infer(world,policy_target,candidate_target);
                   let mut target_verified = false;
                   if let Ok(true) = result {
                       target_verified = true;
                   }
                   if !target_verified {
                       continue;
                   }

                   //check constraint
                   let policy_constraint = prohibit.get_constraint();
                   let mut constraint_verified = false;
                   if let Some(constraint) = policy_constraint {
                       let ret = ConstraintInference::infer(world,constraint);
                       if let Ok(true) = ret {
                           constraint_verified = true;
                       }
                   } else {
                       constraint_verified = true;
                   }

                   if constraint_verified {
                       prohibited = true;
                       break;
                   }
               }
           }
           if prohibited {
               //already permitted, need to check conflict strategy
               return match conflict {
                   ConflictStrategy::perm => {
                       Ok(true)
                   }
                   ConflictStrategy::prohibit => {
                       Ok(false)
                   }
                   ConflictStrategy::invalid => {
                       Ok(false)
                   }
               }
           }
           return Ok(true);
       }

       //here not matched any permission at this level, need to check inheritFrom
       let inheritFrom = policy.get_inheritFrom().clone();
       let inheritFrom = inheritFrom.unwrap();
       for inherit in inheritFrom {
           let inherit_policy = world.get_policy(inherit.to_string());
           if let Some(inherit_policy) = inherit_policy {
               let result = PolicyEngine::eval(world,&inherit_policy,req);
               if let Ok(true) = result {
                   return Ok(true);
               }
           }
       }

       Ok(false)
    }
}

impl Evaluator for Offer {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                // Note: No need to check assignee and assigner here, because offer is not for specific assignee
                // let candidate_assignee = candidate_assignee.clone();
                // let policy_assignee = permission.get_assignee().clone();
                // let mut assignee_verified = false;
                // if candidate_assignee.is_some() && policy_assignee.is_some() {
                //     let union = PartyUnion::Party(policy_assignee.unwrap());
                //     let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                //     if let Ok(true) = ret {
                //         assignee_verified = true;
                //     }
                // }
                // if !assignee_verified {
                //     //no need to check other parts, must have exact one assignee
                //     continue;
                // }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                } else {
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    if !assignee_verified {
                        //no need to check other parts, must have exact one assignee
                        continue;
                    }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    if !assigner_verified {
                        //no need to check other parts, must have exact one assigner
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    } else {
                        constraint_verified = true;
                    }

                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Evaluator for Set {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                let candidate_assignee = candidate_assignee.clone();
                let policy_assignee = permission.get_assignee().clone();
                let mut assignee_verified = false;
                if candidate_assignee.is_some() && policy_assignee.is_some() {
                    let union = policy_assignee.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                    if let Ok(true) = ret {
                        assignee_verified = true;
                    }
                }
                if !assignee_verified {
                    //no need to check other parts, must have exact one assignee
                    continue;
                }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                } else {
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    if !assignee_verified {
                        //no need to check other parts, must have exact one assignee
                        continue;
                    }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    if !assigner_verified {
                        //no need to check other parts, must have exact one assigner
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    }
                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Evaluator for Privacy {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                let candidate_assignee = candidate_assignee.clone();
                let policy_assignee = permission.get_assignee().clone();
                let mut assignee_verified = false;
                if candidate_assignee.is_some() && policy_assignee.is_some() {
                    let union = policy_assignee.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                    if let Ok(true) = ret {
                        assignee_verified = true;
                    }
                }
                if !assignee_verified {
                    //no need to check other parts, must have exact one assignee
                    continue;
                }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                }else {
                    //for no constraint, it is always permitted
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    if !assignee_verified {
                        //no need to check other parts, must have exact one assignee
                        continue;
                    }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    if !assigner_verified {
                        //no need to check other parts, must have exact one assigner
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    } else {
                        constraint_verified = true;
                    }

                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Evaluator for Request {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                let candidate_assignee = candidate_assignee.clone();
                let policy_assignee = permission.get_assignee().clone();
                let mut assignee_verified = false;
                if candidate_assignee.is_some() && policy_assignee.is_some() {
                    let union = policy_assignee.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                    if let Ok(true) = ret {
                        assignee_verified = true;
                    }
                }
                if !assignee_verified {
                    //no need to check other parts, must have exact one assignee
                    continue;
                }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                }else {
                    //for no constraint, just match
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    if !assignee_verified {
                        //no need to check other parts, must have exact one assignee
                        continue;
                    }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    if !assigner_verified {
                        //no need to check other parts, must have exact one assigner
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    } else {
                        constraint_verified = true;
                    }

                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Evaluator for Assert {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                let candidate_assignee = candidate_assignee.clone();
                let policy_assignee = permission.get_assignee().clone();
                let mut assignee_verified = false;
                if candidate_assignee.is_some() && policy_assignee.is_some() {
                    let union = policy_assignee.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                    if let Ok(true) = ret {
                        assignee_verified = true;
                    }
                }
                // if !assignee_verified {
                //     //no need to check other parts, must have exact one assignee
                //     continue;
                // }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                // if !assigner_verified {
                //     //no need to check other parts, must have exact one assigner
                //     continue;
                // }

                //at least one of assignee or assigner must be verified
                if !assignee_verified && !assigner_verified {
                    //both assignee and assigner not verified
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                } else {
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    // if !assignee_verified {
                    //     //no need to check other parts, must have exact one assignee
                    //     continue;
                    // }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    // if !assigner_verified {
                    //     //no need to check other parts, must have exact one assigner
                    //     continue;
                    // }

                    //at least one of assignee or assigner must be verified
                    if !assignee_verified && !assigner_verified {
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    } else {
                        //this is for no constraint in policy
                        constraint_verified = true;
                    }

                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl Evaluator for Ticket {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        let policy = &self.policy;
        let mut conflict = policy.get_conflict().clone();
        if conflict.is_none() {
            conflict = Some(ConflictStrategy::perm);
        }
        let conflict = conflict.unwrap();

        let candidate = req.get_action().clone();
        if candidate.is_none() {
            return Ok(false);
        }

        //check allow permissions
        let permissions = policy.get_permission();
        if permissions.is_none() {
            return Ok(false);
        }
        let prohibitions = policy.get_prohibition();

        /*
         * 1. Check Assigner and Assignee match in each permission
         * 2. Check Target match in each permission
         * 3. Check Action match in each permission
         * 4. Check Constraint match in each permission
         * 5. Check Obligation match in each permission
         * 6. Check Above match in each prohibition
         * 7. Check Conflict Strategy
         */
        let candidate_assignee = req.get_assignee();
        let candidate_assigner = req.get_assigner();
        let candidate_action = req.get_action();

        let mut permitted = false;
        if let Some(permissions) = permissions {
            for permission in permissions {
                // no assignee in ticket
                // let candidate_assignee = candidate_assignee.clone();
                // let policy_assignee = permission.get_assignee().clone();
                // let mut assignee_verified = false;
                // if candidate_assignee.is_some() && policy_assignee.is_some() {
                //     let union = PartyUnion::Party(policy_assignee.unwrap());
                //     let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                //     if let Ok(true) = ret {
                //         assignee_verified = true;
                //     }
                // }
                // if !assignee_verified {
                //     //no need to check other parts, must have exact one assignee
                //     continue;
                // }

                //do assigner verification
                let candidate_assigner = candidate_assigner.clone();
                let policy_assigner = permission.get_assigner().clone();
                let mut assigner_verified = false;
                let assigner = permission.get_assigner();
                if candidate_assigner.is_some() && assigner.is_some() {
                    let union = policy_assigner.unwrap();
                    let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                    if let Ok(true) = ret {
                        assigner_verified = true;
                    }
                }
                if !assigner_verified {
                    //no need to check other parts, must have exact one assigner
                    continue;
                }

                //do action verification
                let candidate_action = candidate_action.clone().unwrap();
                let policy_action = permission.get_action().clone().unwrap();
                let result = ActionInferencer::infer(world,policy_action,candidate_action);
                let mut action_verified = false;
                if let Ok(true) = result {
                    action_verified = true;
                }
                if !action_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //do target verification
                let candidate_target = req.get_target().clone().unwrap();
                let policy_target = permission.get_target().clone().unwrap();
                let result = AssetInferencer::infer(world,policy_target,candidate_target);
                let mut target_verified = false;
                if let Ok(true) = result {
                    target_verified = true;
                }
                if !target_verified {
                    //no need to check other parts, must have exact one action
                    continue;
                }

                //check constraint
                let policy_constraint = permission.get_constraint();
                let mut constraint_verified = false;
                if let Some(constraint) = policy_constraint {
                    let ret = ConstraintInference::infer(world,constraint);
                    if let Ok(true) = ret {
                        constraint_verified = true;
                    }
                } else {
                    constraint_verified = true;
                }

                if constraint_verified {
                    //every thing ok here, a permission is matched already
                    permitted = true;
                    break;
                }
            }
        }

        //check prohibition
        if permitted {
            let mut prohibited = false;
            if let Some(prohibits) = prohibitions {
                for prohibit in prohibits {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = prohibit.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = policy_assignee.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(true) = ret {
                            assignee_verified = true;
                        }
                    }
                    if !assignee_verified {
                        //no need to check other parts, must have exact one assignee
                        continue;
                    }

                    //do assigner verification
                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = prohibit.get_assigner().clone();
                    let mut assigner_verified = false;
                    let assigner = prohibit.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = policy_assigner.unwrap();
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(true) = ret {
                            assigner_verified = true;
                        }
                    }
                    if !assigner_verified {
                        //no need to check other parts, must have exact one assigner
                        continue;
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone().unwrap();
                    let policy_action = prohibit.get_action().clone().unwrap();
                    let result = ActionInferencer::infer(world,policy_action,candidate_action);
                    let mut action_verified = false;
                    if let Ok(true) = result {
                        action_verified = true;
                    }

                    if !action_verified {
                        //no need to check other parts, must have exact one action
                        continue;
                    }

                    //do target verification
                    let candidate_target = req.get_target().clone().unwrap();
                    let policy_target = prohibit.get_target().clone().unwrap();
                    let result = AssetInferencer::infer(world,policy_target,candidate_target);
                    let mut target_verified = false;
                    if let Ok(true) = result {
                        target_verified = true;
                    }
                    if !target_verified {
                        continue;
                    }

                    //check constraint
                    let policy_constraint = prohibit.get_constraint();
                    let mut constraint_verified = false;
                    if let Some(constraint) = policy_constraint {
                        let ret = ConstraintInference::infer(world,constraint);
                        if let Ok(true) = ret {
                            constraint_verified = true;
                        }
                    } else {
                        constraint_verified = true;
                    }

                    if constraint_verified {
                        prohibited = true;
                        break;
                    }
                }
            }
            if prohibited {
                //already permitted, need to check conflict strategy
                return match conflict {
                    ConflictStrategy::perm => {
                        Ok(true)
                    }
                    ConflictStrategy::prohibit => {
                        Ok(false)
                    }
                    ConflictStrategy::invalid => {
                        Ok(false)
                    }
                }
            }
            return Ok(true);
        }

        //here not matched any permission at this level, need to check inheritFrom
        let inheritFrom = policy.get_inheritFrom().clone();
        let inheritFrom = inheritFrom.unwrap();
        for inherit in inheritFrom {
            let inherit_policy = world.get_policy(inherit.to_string());
            if let Some(inherit_policy) = inherit_policy {
                let result = PolicyEngine::eval(world,&inherit_policy,req);
                if let Ok(true) = result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}




pub struct PolicyEngine;

impl PolicyEngine {
    pub fn find_world_key(policy: &PolicyUnion) -> Option<IriBuf> {
        match policy {
            PolicyUnion::Privacy(p) => {
                return p.policy.get_uid().clone();
            }
            PolicyUnion::Request(r) => {
                return  r.policy.get_uid().clone();
            }
            PolicyUnion::Assert(a) => {
                return  a.policy.get_uid().clone();
            }
            PolicyUnion::Set(s) => {
                return  s.policy.get_uid().clone();
            }
            PolicyUnion::Agreement(p) => {
                return  p.policy.get_uid().clone();
            }
            PolicyUnion::Offer(o) => {
                return  o.policy.get_uid().clone();
            }
            PolicyUnion::Ticket(s) => {
                return  s.policy.get_uid().clone();
            }
        }
    }

    pub fn eval(world: &mut StateWorld, policy: &PolicyUnion,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        match policy {
            PolicyUnion::Privacy(p) => {
                return  p.eval(world,req);
            }
            PolicyUnion::Request(r) => {
                return  r.eval(world,req);
            }
            PolicyUnion::Assert(a) => {
                return  a.eval(world,req);
            }
            PolicyUnion::Set(s) => {
                return  s.eval(world,req);
            }
            PolicyUnion::Agreement(p) => {
                return  p.eval(world,req);
            }
            PolicyUnion::Offer(o) => {
                return  o.eval(world,req);
            }
            PolicyUnion::Ticket(s) => {
                return  s.eval(world,req);
            }
        }
    }
}
