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
use lombok::{Builder, Getter, GetterMut};
use crate::model::rule::Rule;
use crate::model::metadata::Metadata;
use crate::reference::types::ConflictTerm;
use crate::traits::validate::Validate;

use crate::model::error::OdrlError;
use crate::reference::types::PolicyClassType;

#[derive(Debug,Builder,Getter,GetterMut,Clone)]
pub struct Policy {
    //Policy class type
    pub class: Option<PolicyClassType>,
    //Policy must have a unique identifier
    pub uid: Option<IriBuf>,
    //Policy must have at least one rule, permission, prohibition, or obligation
    pub rules : Option<Vec<Rule>>,
    //Policy may have none,one or many profile
    pub profile: Option<Vec<IriBuf>>,
    //Policy may have none,one or many inheritFrom to identify the parent policy
    pub inherit_from: Option<Vec<IriBuf>>,
    //Policy may have none,one or many 
    pub conflicts: Option<ConflictTerm>,

    //Shared targets to all rules
    pub targets: Option<Vec<IriBuf>>,
    //Shared Assigner
    pub assigner: Option<Vec<IriBuf>>,
    //Shared Assignee
    pub assignee: Option<Vec<IriBuf>>,
    //Shared Action
    pub actions: Option<Vec<IriBuf>>,

    //Meta
    pub metadata: Option<Metadata>,
}

impl Default for Policy {
    fn default() -> Self {
        Self { 
            class: None,
            uid: Default::default(), 
            rules: Default::default(), 
            profile: Default::default(), 
            inherit_from: Default::default(),
            conflicts: Default::default(),
            targets: Default::default(),
            assigner: Default::default(),
            assignee: Default::default(),
            actions: Default::default(),
            metadata: Default::default()
        }
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
        Policy {
            class:None,
            uid: None,
            rules: None,
            profile: None,
            inherit_from: None,
            conflicts: Some(ConflictTerm::Invalid),
            targets: Default::default(),
            assigner: Default::default(),
            assignee: Default::default(),
            actions: Default::default(),
            metadata: Default::default()
        }
    }

    pub fn with_class(class: PolicyClassType, uid: IriBuf) -> Self {
        let mut builder = Policy::builder();
        builder.uid(Some(uid));
        builder.class(Some(class));
        builder.build()
    }

    pub fn validate_class_set(&self) -> Result<(), OdrlError> {
        //must have at least one rule
        let rules = self.get_rules();
        match rules {
            Some(rlist) =>{
            if  rlist.len() == 0 {
                    return Err(OdrlError::NoneRuleDefinition);
            }
            },
            None => {
                return Err(OdrlError::InvalidRuleDefinition);
            }
        }
        Ok(())
    }

    pub fn validate_class_offer(&self) -> Result<(), OdrlError> {
        let mut target_found = false;
        let mut assigner_found = false;
        //must have at least one rule
        let rules = self.get_rules();
        match rules {
            Some(rlist) =>{
                if  rlist.len() == 0 {
                    return Err(OdrlError::NoneRuleDefinition);
                }

                for rule in rlist {
                    if rule.get_target().is_some() {
                        target_found = true;
                    }

                    if rule.get_assigner().is_some() {
                        assigner_found = true;
                    }

                    if target_found && assigner_found {
                        break;
                    }
                }
            },
            None => {
                return Err(OdrlError::InvalidRuleDefinition);
            }
        }

        //must have at least one target
        if let Some(targets) = self.get_targets() {
            if targets.len() >= 1 {
                target_found = true;
            }
        }

        if !target_found {
            return Err(OdrlError::MissingOfferTarget);
        }

        //must have at least assigner
        if let Some(assigner) = self.get_assigner() {
            if assigner.len() > 0 {
                assigner_found = true;
            }
        }

        if !assigner_found {
            return Err(OdrlError::MissingOfferAssigner);
        }

        Ok(())
    }

    pub fn validate_class_agreement(&self) -> Result<(), OdrlError> {
        let mut target_found = false;
        let mut assigner_found = false;
        let mut assignee_found = false;

        //must have at least one rule
        let rules = self.get_rules();
        match rules {
            Some(rlist) =>{
                if  rlist.len() == 0 {
                    return Err(OdrlError::NoneRuleDefinition);
                }

                for rule in rlist {
                    if rule.get_target().is_some() {
                        target_found = true;
                    }

                    if rule.get_assigner().is_some() {
                        assigner_found = true;
                    }

                    if rule.get_assignee().is_some() {
                        assignee_found = true;
                    }

                    if target_found && assigner_found && assignee_found {
                        break;
                    }
                }
            },
            None => {
                return Err(OdrlError::InvalidRuleDefinition);
            }
        }

        //must have at least one target
        if let Some(targets) = self.get_targets() {
            if targets.len() >= 1 {
                target_found = true;
            }
        }

        if !target_found {
            return Err(OdrlError::MissingAgreementTarget);
        }

        //must have at least assigner
        if let Some(assigner) = self.get_assigner() {
            if assigner.len() > 0 {
                assigner_found = true;
            }
        }

        if !assigner_found {
            return Err(OdrlError::MissingAgreementAssigner);
        }

        //must have at least assignee
        if let Some(assignee) = self.get_assignee()  {
            if assignee.len() > 0 {
                assignee_found = true;
            }
        }

        if !assignee_found {
            return Err(OdrlError::MissingAgreementAssignee);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::rule::Rule;
    use crate::reference::types::PolicyClassType;

    use super::*;

    #[test]
    fn test_policy_new() {
        let mut builder = Policy::builder();
        builder.uid(Some(IriBuf::new("http://example.com/policy".to_owned()).unwrap()));


        let profiles = vec![IriBuf::new("http://data.com/profile".to_owned()).unwrap()];
        builder.profile(Some(profiles));

        let rules = vec![Rule::new()];
        builder.rules(Some(rules));

        let policy = builder.build();
        let result = policy.validate();
        match result {
            Ok(()) => println!("Policy is valid"),
            Err(e) => println!("Error: {:?}", e.to_string()),
        }
    }

    #[test]
    fn test_policy_builder(){
        let mut builder = Policy::builder();
        builder.uid(Some(IriBuf::new("http://example.com/policy".to_owned()).unwrap()));

        let profiles = vec![IriBuf::new("http://data.com/profile".to_owned()).unwrap()];
        builder.profile(Some(profiles));
        let policy = builder.build();
                                
        assert_eq!(policy.uid.unwrap(),"http://example.com/policy".to_owned());       
        let profiles = vec![IriBuf::new("http://data.com/profile".to_owned()).unwrap()];
        assert_eq!(policy.profile.unwrap(),profiles);               
    }

    #[test]
    fn test_policy_new_with_class() {
        let policy = Policy::with_class(PolicyClassType::SET, IriBuf::new("http://example.com/policy".to_owned()).unwrap());
        
        let class = policy.get_class().as_ref().unwrap();
        assert_eq!(*class, PolicyClassType::SET);

        let result = policy.validate();
        match result {
            Ok(()) => println!("Policy is valid"),
            Err(e) => println!("Error: {:?}", e.to_string()),
        }
    }
}