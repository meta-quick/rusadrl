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

pub enum ReferenceType {
    PolicySet,
    PolicyRequest,
    PolicyOffer,
    PolicyCC,
    PolicyAgreement,
    PolicyTicket,
    PolicyAssertion,
    PolicyPrivacy,
}

impl ReferenceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReferenceType::PolicySet => "http://www.w3.org/ns/odrl/2/Set",
            ReferenceType::PolicyRequest => "http://www.w3.org/ns/odrl/2/Request",
            ReferenceType::PolicyOffer => "http://www.w3.org/ns/odrl/2/Offer",
            ReferenceType::PolicyCC => "http://www.w3.org/ns/odrl/2/CC",
            ReferenceType::PolicyAgreement => "http://www.w3.org/ns/odrl/2/Agreement",
            ReferenceType::PolicyTicket => "http://www.w3.org/ns/odrl/2/Ticket",
            ReferenceType::PolicyAssertion => "http://www.w3.org/ns/odrl/2/Assertion",
            ReferenceType::PolicyPrivacy => "http://www.w3.org/ns/odrl/2/Privacy",
        }
    }
}

#[derive(Debug,Default,PartialEq, Eq, Clone)]
pub enum RuleType {
    #[default]
    Permission,
    Prohibition,
    Duty,
}

impl RuleType {
    pub const RULE_PERMISSION: i32 = 0;
    pub const RULE_PROHIBITION: i32 = 1;
    pub const RULE_DUTY: i32 = 2;
}
