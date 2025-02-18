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
