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
use crate::model::action::Action;
use crate::model::asset::Asset;
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint::Constraint;
use crate::model::duty::Duty;
use crate::model::metadata::Metadata;
use crate::traits::traits::Validate;

use crate::model::error::OdrlError;
use crate::model::party::Party;
use crate::model::permission::Permission;
use crate::model::prohibition::Prohibition;

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

    //Meta
    pub metadata: Option<Metadata>,
}

impl Validate for Policy {
    fn validate(& mut self) -> Result<(), OdrlError> {
        //verify if uid is valid
        if self.uid.is_none() {
            return Err(OdrlError::InvalidIri);
        }

        let permission = self.get_permission();
        let prohibition = self.get_prohibition();
        let obligation = self.get_obligation();
        if permission.is_none() && prohibition.is_none() && obligation.is_none() {
            return Err(OdrlError::InvalidRuleDefinition);
        }
        //check conflict
        let conflict = self.get_conflict();
        if conflict.is_none() {
            self.set_conflict(Some(ConflictStrategy::perm));
        }

        Ok(())
    }
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

impl Validate for Agreement {
    fn validate(&mut self) -> Result<(), OdrlError> {
         /*
          *  {
          *       "@context": "http://www.w3.org/ns/odrl.jsonld",
          *       "@type": "Agreement",
          *       "uid": "http://example.com/policy:1012",
          *       "profile": "http://example.com/odrl:profile:01",
          *       "permission": [{
          *           "target": "http://example.com/asset:9898.movie",
          *           "assigner": "http://example.com/party:org:abc",
          *           "assignee": "http://example.com/party:person:billie",
          *           "action": "play"
          *       }]
          *   }
          */
          let result = self.policy.validate();
          if result.is_err() {
              return result;
          }

          let common_assignee = self.policy.get_assignee().clone();
          let common_assigner = self.policy.get_assigner().clone();
          let common_target = self.policy.get_target().clone();

          let permissions = self.policy.get_permission_mut();
          let mut has_permission = false;
          if permissions.is_some() {
             //check if permission has assignee, assigner, target
             for permission in permissions.as_mut().unwrap() {
                 if permission.get_assignee().is_none() {
                     if common_assignee.is_none() {
                         return Err(OdrlError::MissingAgreementAssignee);
                     }
                     permission.set_assignee(common_assignee.clone());
                 }

                 if permission.get_assigner().is_none() {
                     if common_assigner.is_none() {
                         return Err(OdrlError::MissingAgreementAssigner);
                     }
                     permission.set_assigner(common_assigner.clone());
                 }

                 if permission.get_target().is_none() {
                     if common_target.is_none() {
                         return Err(OdrlError::MissingAgreementTarget);
                     }
                     permission.set_target(common_target.clone());
                 }

                 has_permission = true;
             }
         }

         let mut has_obligation = false;
         let obligations = self.policy.get_obligation_mut();
         if obligations.is_some() {
             for mut obligation in obligations.as_mut().unwrap() {
                 if obligation.get_assignee().is_none() {
                     if common_assignee.is_none() {
                         return Err(OdrlError::MissingAgreementAssignee);
                     }
                     obligation.set_assignee(common_assignee.clone());
                 }
                 if obligation.get_assigner().is_none() {
                     if common_assigner.is_none() {
                         return Err(OdrlError::MissingAgreementAssigner);
                     }
                     obligation.set_assigner(common_assigner.clone());
                 }
                 if obligation.get_target().is_none() {
                     if common_target.is_none() {
                         return Err(OdrlError::MissingAgreementTarget);
                     }
                     obligation.set_target(common_target.clone());
                 }
                 has_obligation = true;
             }
         }

         let mut has_prohibition = false;
        let prohibitions = self.policy.get_prohibition_mut();
         if prohibitions.is_some() {
            for mut prohibition in prohibitions.as_mut().unwrap() {
                if prohibition.get_assignee().is_none() {
                    if common_assignee.is_none() {
                        return Err(OdrlError::MissingAgreementAssignee);
                    }
                    prohibition.set_assignee(common_assignee.clone());
                }
                if prohibition.get_assigner().is_none() {
                    if common_assigner.is_none() {
                        return Err(OdrlError::MissingAgreementAssigner);
                    }
                    prohibition.set_assigner(common_assigner.clone());
                }
                if prohibition.get_target().is_none() {
                    if common_target.is_none() {
                        return Err(OdrlError::MissingAgreementTarget);
                    }
                    prohibition.set_target(common_target.clone());
                }
                has_prohibition = true;
            }
         }
         Ok(())
    }
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

#[cfg(test)]
mod tests {
}