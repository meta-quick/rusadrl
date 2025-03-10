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
use lombok::{Builder, Getter, GetterMut};
use crate::model::action::Action;
use crate::model::asset::Asset;
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint::Constraint;
use crate::model::duty::Duty;
use crate::model::rule::Rule;
use crate::model::metadata::Metadata;
use crate::traits::validate::Validate;

use crate::model::error::OdrlError;
use crate::model::party::Party;
use crate::model::permission::Permission;
use crate::model::prohibition::Prohibition;
use crate::reference::types::PolicyClassType;

//Identifier:	http://www.w3.org/ns/odrl/2/Policy
#[derive(Debug,Default,Builder,Getter,GetterMut,Clone)]
pub struct Policy {
    //Policy must have a unique identifier
    pub uid: Option<IriBuf>,
    pub profile: Option<Vec<IriBuf>>,

    pub action: Option<Action>,
    pub assignee: Option<Vec<Party>>,
    pub assigner: Option<Vec<Party>>,
    pub conflict: Option<ConflictStrategy>,
    pub permission: Option<Vec<Permission>>,
    pub prohibition: Option<Vec<Prohibition>>,
    pub obligation: Option<Vec<Duty>>,
    pub target: Option<Vec<Asset>>,
    pub inheritFrom : Option<Vec<IriBuf>>,
    pub constraint: Option<Vec<Constraint>>,
    pub relation: Option<Vec<IriBuf>>,
    pub function: Option<Vec<IriBuf>>,

    //Meta
    pub metadata: Option<Metadata>,
}

impl Default for Policy {
    fn default() -> Self {
        Self::default()
    }
}

impl Validate for Policy {
    fn validate(&self) -> Result<(), OdrlError> {
        //verify if uid is valid
        if self.uid.is_none() {
            return Err(OdrlError::InvalidIri);
        }

        /* verify policy per class type
         * 1. NONE/SET is the default one, Set Policy subclass is also the default subclass of Policy;
         *    Set represents any combination of Rule
         */

        let class = self.get_class();
        match class {
            Some(class) => {
                match class {
                    /*
                     *  {
                     *      "@context": "http://www.w3.org/ns/odrl.jsonld",
                     *      "@type": "Set",
                     *       "uid": "http://example.com/policy:1010",
                     *       "permission": [{
                     *           "target": "http://example.com/asset:9898.movie",
                     *           "action": "use"
                     *       }]
                     *   }
                     */
                    PolicyClassType::SET => {
                        self.validate_class_set()
                    },
                    /*
                     *  {
                     *       "@context": "http://www.w3.org/ns/odrl.jsonld",
                     *       "@type": "Offer",
                     *       "uid": "http://example.com/policy:1011",
                     *       "profile": "http://example.com/odrl:profile:01",
                     *       "permission": [{
                     *           "target": "http://example.com/asset:9898.movie",
                     *           "assigner": "http://example.com/party:org:abc",
                     *           "action": "play"
                     *       }]
                     *   }
                     */
                    PolicyClassType::OFFER => {
                        self.validate_class_offer()
                    },
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
                    PolicyClassType::AGREEMENT => {
                        self.validate_class_agreement()
                    },
                    PolicyClassType::NONE => {
                        self.validate_class_set()
                    },
                }
            },
            None => { //As default, policy is a set policy
                self.validate_class_set()
            }
        }
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