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
use crate::model::asset::Asset;
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint::Constraint;
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
    pub assignee: Option<Party>,
    pub assigner: Option<Party>,
    pub conflict: Option<ConflictStrategy>,
    pub permission: Option<Vec<Permission>>,
    pub prohibition: Option<Vec<Prohibition>>,
    pub obligation: Option<Vec<Duty>>,
    pub target: Option<Asset>,
    pub inheritFrom : Option<Vec<IriBuf>>,
    pub constraint: Option<Vec<Constraint>>,
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

#[derive(Debug,Default,Builder,Clone)]
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

        if let Some(permissions) = permissions {
            for permission in permissions {
                let assignee = permission.get_assignee();
                if let Some(assignee) = assignee {
                    let candidate_assignee = candidate_assignee.clone();
                    let policy_assignee = permission.get_assignee().clone();

                    //do assignee verification
                    let mut assignee_verified = false;
                    if candidate_assignee.is_some() && policy_assignee.is_some() {
                        let union = PartyUnion::Party(policy_assignee.unwrap());
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assignee.unwrap());
                        if let Ok(ret) = ret {
                            assignee_verified = ret;
                        }
                    }

                    let candidate_assigner = candidate_assigner.clone();
                    let policy_assigner = permission.get_assigner().clone();

                    //do assigner verification
                    let assigner_verified = false;
                    let assigner = permission.get_assigner();
                    if candidate_assigner.is_some() && assigner.is_some() {
                        let union = PartyUnion::Party(policy_assigner.unwrap());
                        let ret = PartyInferencer::infer_party(world,&union,&candidate_assigner.unwrap());
                        if let Ok(ret) = ret {
                            assignee_verified = ret;
                        }
                    }

                    //do action verification
                    let candidate_action = candidate_action.clone();
                    let policy_action = permission.get_action().clone();

                    ActionInferencer::infer_action(world,conflict,&policy_action,&candidate_action);
                }


                let assigner = permission.get_assigner();
                let target = permission.get_target();
            }
        }


       //handle inheritFrom
       let inhirts = policy.get_inheritFrom();
       //TODO: do loop inheritFrom check
       if let Some(inherits) = inhirts {
           for inherit in inherits {
               let inherit_policy = world.get_policy(inherit.to_string());
               if let Some(inherit_policy) = inherit_policy {
                   let result = PolicyEngine::eval(world,inherit_policy,req);
               }
           }
       }

       Ok(true)
    }
}

impl Evaluator for Offer {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

impl Evaluator for Set {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

impl Evaluator for Privacy {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

impl Evaluator for Request {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

impl Evaluator for Assert {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

impl Evaluator for Ticket {
    fn eval(&self,world: &mut StateWorld,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}




pub struct PolicyEngine;

impl PolicyEngine {
    pub fn eval(world: &mut StateWorld, policy: PolicyUnion,req: &OdrlRequest) -> Result<bool, anyhow::Error> {
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
