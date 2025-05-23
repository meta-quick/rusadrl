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

use lombok::{Builder, Getter, GetterMut, Setter};
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
        //find the last /
        let index = value.rfind("/").unwrap();
        let value = value.split_at(index+1).1.to_string();

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
            "use" => Ok(ActionType::Use),
            "transfer" => Ok(ActionType::Transfer),
            _ => Err(format!("Invalid action type: {}", value))
        }
    } 
}

#[derive(Debug,Default,Clone)]
pub struct ActionInferencer;
impl ActionInferencer {
   pub fn infer(world: &mut StateWorld, action: Action,candidate: Action) -> Result<bool, anyhow::Error> {
       let mut permitted = false;


       if action.actionType == candidate.actionType {
           permitted = true;
       } else {
           /* check includedIn
            * if candidate parent is the same as action, then it is permitted
            * ex: candidate includedIn is 'use', and action is 'use' also, and 'use' granted/prohibited, then match
            */
           if let Some(includedIn) = candidate.get_includedIn() {
               for included in includedIn {
                   if included.actionType == action.actionType  {
                       permitted = true;
                       break;
                   }
               }
           }

           if !permitted {
               /* check implies
                * if candidate is in implies of the action, then it is permitted
                */
               if let Some(impls) = action.get_implies() {
                   for impls in impls {
                       if impls.actionType == candidate.actionType {
                           permitted = true;
                           break;
                       }
                   }
               }
           }
       }

       if !permitted {
           return Ok(false);
       }

       //check refinement
       let mut refined = true;
       if let Some(refinement) = action.get_refinements() {
           for refinement in refinement {
               match refinement {
                   ConstraintUnion::Constraint(constraint) => {
                       let result = constraint.eval(world);
                       match result {
                           Ok(false) => {
                               refined = false;
                           },
                           _ => {
                           }
                       }
                   }
                   ConstraintUnion::LogicConstraint(ac) => {
                       let result = ac.eval(world);
                       match result {
                           Ok(false) => {
                               refined = false;
                           },
                           _ => {
                           }
                       }
                   }
               }
           }
       }

       //action match already, just check refinement
       Ok(refined)
   }
}