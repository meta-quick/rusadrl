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

use anyhow::{anyhow, Error};
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::conflict_strategy::ConflictStrategy;
use crate::model::constraint::{ConstraintUnion};
use crate::model::metadata::Metadata;
use crate::model::stateworld::StateWorld;
use crate::traits::definions::LogicEval;

#[derive(Debug,Clone,PartialEq)]
pub enum ActionType {
    //http://www.w3.org/ns/odrl/2/acceptTracking
    AcceptTracking,
    //http://www.w3.org/ns/odrl/2/aggregate
    Aggregate,
    //http://www.w3.org/ns/odrl/2/annotate
    Annotate,
    //http://www.w3.org/ns/odrl/2/anonymize
    Anonymize,
    //http://www.w3.org/ns/odrl/2/archive
    Archive,
    //http://www.w3.org/ns/odrl/2/attribute
    Attribute,
    //http://creativecommons.org/ns#Attribution
    Attribution,
    //http://creativecommons.org/ns#CommericalUse
    CommericalUse,
    //http://www.w3.org/ns/odrl/2/compensate
    Compensate,
    //http://www.w3.org/ns/odrl/2/concurrentUse
    ConcurrentUse,
    //http://www.w3.org/ns/odrl/2/delete
    Delete,
    //http://www.w3.org/ns/odrl/2/derive
    Derive,
    //http://creativecommons.org/ns#DerivativeWorks
    Derivative,
    //http://www.w3.org/ns/odrl/2/digitize
    Digitize,
    //http://www.w3.org/ns/odrl/2/display
    Display,
    //http://www.w3.org/ns/odrl/2/distribute
    Distribute,
    //	http://creativecommons.org/ns#Distribution
    Distribution,
    //http://www.w3.org/ns/odrl/2/ensureExclusivity
    EnsureExclusivity,
    //http://www.w3.org/ns/odrl/2/execute
    Execute,
    //http://www.w3.org/ns/odrl/2/extract
    Extract,
    //http://www.w3.org/ns/odrl/2/give
    Give,
    //http://www.w3.org/ns/odrl/2/grantUse
    GrantUse,
    //http://www.w3.org/ns/odrl/2/include
    Include,
    //http://www.w3.org/ns/odrl/2/index
    Index,
    //http://www.w3.org/ns/odrl/2/inform
    Inform,
    //http://www.w3.org/ns/odrl/2/install
    Install,
    //http://www.w3.org/ns/odrl/2/modifiy
    Modify,
    //http://www.w3.org/ns/odrl/2/move
    Move,
    //http://www.w3.org/ns/odrl/2/nextPolicy
    NextPolicy,
    //http://creativecommons.org/ns#Notice
    Notice,
    //http://www.w3.org/ns/odrl/2/obtainConsent
    ObtainConsent,
    //http://www.w3.org/ns/odrl/2/play
    Play,
    //http://www.w3.org/ns/odrl/2/present
    Present,
    //http://www.w3.org/ns/odrl/2/print
    Print,
    //http://www.w3.org/ns/odrl/2/read
    Read,
    //http://www.w3.org/ns/odrl/2/reproduce
    Reproduce,
    //http://creativecommons.org/ns#Reproduction
    Reproduction,
    //http://www.w3.org/ns/odrl/2/reviewPolicy
    ReviewPolicy,
    //http://www.w3.org/ns/odrl/2/sell
    Sell,
    //http://creativecommons.org/ns#ShareAlike
    ShareAlike,
    //http://creativecommons.org/ns#Sharing
    Sharing,
    //http://creativecommons.org/ns#SourceCode
    SourceCode,
    //http://www.w3.org/ns/odrl/2/stream
    Stream,
    //http://www.w3.org/ns/odrl/2/synchronize
    Synchronize,
    //http://www.w3.org/ns/odrl/2/textToSpeech
    TextToSpeech,
    //http://www.w3.org/ns/odrl/2/transform
    Transform,
    //http://www.w3.org/ns/odrl/2/translate
    Translate, 
    //http://www.w3.org/ns/odrl/2/uninstall
    Uninstall,
    //http://www.w3.org/ns/odrl/2/watermark
    Watermark,
    //top level action
    Use,
    Transfer
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::AcceptTracking
    }
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Action {
    pub actionType: ActionType,
    //An action must have one IncludedIn except use and transfer
    pub includedIn: Option<Vec<Action>>,
    pub implies: Option<Vec<Action>>,
    pub refinements: Option<Vec<ConstraintUnion>>,
    pub metadata: Metadata,
}

impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<&str> for ActionType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "acceptTracking" => Ok(ActionType::AcceptTracking),
            "aggregate" => Ok(ActionType::Aggregate),
            "annotate" => Ok(ActionType::Annotate),
            "anonymize" => Ok(ActionType::Anonymize),
            "archive" => Ok(ActionType::Archive),
            "attribute" => Ok(ActionType::Attribute),
            "attribution" => Ok(ActionType::Attribution),
            "commericalUse" => Ok(ActionType::CommericalUse),
            "compensate" => Ok(ActionType::Compensate),
            "concurrentUse" => Ok(ActionType::ConcurrentUse),
            "delete" => Ok(ActionType::Delete),
            "derive" => Ok(ActionType::Derive),
            "derivative" => Ok(ActionType::Derivative),
            "digitize" => Ok(ActionType::Digitize),
            "display" => Ok(ActionType::Display),
            "distribute" => Ok(ActionType::Distribute),
            "distribution" => Ok(ActionType::Distribution),
            "ensureExclusivity" => Ok(ActionType::EnsureExclusivity),
            "execute" => Ok(ActionType::Execute),
            "extract" => Ok(ActionType::Extract),
            "give" => Ok(ActionType::Give),
            "grantUse" => Ok(ActionType::GrantUse),
            "include" => Ok(ActionType::Include),
            "index" => Ok(ActionType::Index),
            "inform" => Ok(ActionType::Inform),
            "install" => Ok(ActionType::Install),
            "modify" => Ok(ActionType::Modify),
            "move" => Ok(ActionType::Move),
            "nextPolicy" => Ok(ActionType::NextPolicy),
            "notice" => Ok(ActionType::Notice),
            "obtainConsent" => Ok(ActionType::ObtainConsent),
            "play" => Ok(ActionType::Play),
            "present" => Ok(ActionType::Present),
            "print" => Ok(ActionType::Print),
            "read" => Ok(ActionType::Read),
            "reproduce" => Ok(ActionType::Reproduce),
            "reproduction" => Ok(ActionType::Reproduction),
            "reviewPolicy" => Ok(ActionType::ReviewPolicy),
            "sell" => Ok(ActionType::Sell),
            "shareAlike" => Ok(ActionType::ShareAlike),
            "sharing" => Ok(ActionType::Sharing),
            "sourceCode" => Ok(ActionType::SourceCode),
            "stream" => Ok(ActionType::Stream),
            "synchronize" => Ok(ActionType::Synchronize),
            "textToSpeech" => Ok(ActionType::TextToSpeech),
            "transform" => Ok(ActionType::Transform),
            "translate" => Ok(ActionType::Translate),
            "uninstall" => Ok(ActionType::Uninstall),
            "watermark" => Ok(ActionType::Watermark),
            _ => Err(format!("Invalid action type: {}", value))
        }
    } 
}

#[derive(Debug,Default,Clone)]
pub struct ActionInferencer;
impl ActionInferencer {
   pub fn infer_action(world: &mut StateWorld,strategy: ConflictStrategy,permissions: Option<Vec<Action>>, prohibitions: Option<Vec<Action>>, candidate: Action) -> Result<bool, anyhow::Error> {
       //check if the action is in the permission list
       let mut permited = false;
       let mut permitted_action: Option<Action> = None;
       if let Some(perms) = permissions {
           for perm in perms {
               //action type is the same
               if perm.actionType == candidate.actionType {
                   permited = true;
                   permitted_action = Some(perm.clone());
                   break;
               }
               if let Some(includes) = candidate.get_includedIn() {
                   for incl in includes {
                        if incl.actionType == perm.actionType {
                            permited = true;
                            permitted_action = Some(incl.clone());
                            break;
                        }
                   }
               }
               if let Some(implies) = candidate.get_implies() {
                   for impls in implies {
                       if impls.actionType == perm.actionType {
                           permited = true;
                           permitted_action = Some(impls.clone());
                           break;
                       }
                   }
               }
           }
       }

       let mut prohibed = false;
       let mut prohibed_action: Option<Action> = None;
       if let Some(prohibitions) = prohibitions {
           for prohibition in prohibitions {
               if prohibition.actionType == candidate.actionType {
                   prohibed = true;
                   prohibed_action = Some(prohibition.clone());
                   break;
               }

               if let Some(includes) = candidate.get_includedIn() {
                   for incl in includes {
                        if incl.actionType == prohibition.actionType {
                            prohibed = true;
                            prohibed_action = Some(prohibition.clone());
                            break;
                        }
                   }
               }

               if let Some(implies) = candidate.get_implies() {
                   for impls in implies {
                       if impls.actionType == prohibition.actionType {
                           prohibed = true;
                           prohibed_action = Some(impls.clone());
                           break;
                       }
                   }
               }
           }
       }

       /*
        * Refinement logic:
        *  1. Permit refinements should be true
        *  2. Prohibit refinements should be false
        */

       //check refinements
       let mut permit_refined = true;
       if let Some(perm) = permitted_action {
           if let Some(constraints) = perm.get_refinements() {
               for constraint in constraints {
                   match constraint {
                       ConstraintUnion::Constraint(constraint) => {
                           let result = constraint.eval(world);
                           match result {
                               Ok(false) => {
                                   permit_refined = false;
                               }
                               _ => {}
                           }
                       }
                       ConstraintUnion::LogicConstraint(ac) => {
                           let result = ac.eval(world);
                           match result {
                               Ok(false) => {
                                   permit_refined = false;
                               }
                               _ => {}
                           }
                       }
                   }
               }
           }
       }

       let mut prohibit_refined = false;
       if let Some(prohibit) = prohibed_action {
           if let Some(constraints) = prohibit.get_refinements() {
               for constraint in constraints {
                   match constraint {
                       ConstraintUnion::Constraint(constraint) => {
                           let result = constraint.eval(world);
                           match result {
                               Ok(true) => {
                                   prohibit_refined = true;
                               }
                               _ => {}
                           }
                       }
                       ConstraintUnion::LogicConstraint(ac) => {
                           let result = ac.eval(world);
                           match result {
                               Ok(false) => {
                                   prohibit_refined = true;
                               }
                               _ => {}
                           }
                       }
                   }
               }
           }
       }

       //in permission and no in prohibition and permission refinement is true
       if (permited && permit_refined && !prohibed ) ||
           ( permited && permit_refined && prohibed && !prohibit_refined )  {
           return Ok(true);
       }

       //in permission and in prohibition and refinement is true
       if permited && permit_refined && prohibed && prohibit_refined {
           match strategy {
               ConflictStrategy::perm => {
                   return Ok(true);
               },
               ConflictStrategy::prohibit => {
                   return Ok(false);
               },
               ConflictStrategy::invalid => {
                   return Err(anyhow!("Invalid action"));
               }
           }
       }

       //permission refinement is false
       if !permit_refined {
           return Ok(false);
       }

       return Err(anyhow!("Invalid action"));
   }
}