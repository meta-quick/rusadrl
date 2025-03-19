use crate::model::policy::{Agreement, Assert, Offer, PolicyUnion, Privacy, Request, Set, Ticket};

pub struct  ModelFactory;

impl ModelFactory {
    pub fn create(ty: String) -> PolicyUnion{
        match ty.as_str() {
           "http://www.w3.org/ns/odrl/2/Agreement" => PolicyUnion::Agreement(Agreement::default()),
           "http://www.w3.org/ns/odrl/2/Request" => PolicyUnion::Request(Request::default()),
           "http://www.w3.org/ns/odrl/2/Ticket" => PolicyUnion::Ticket(Ticket::default()),
           "http://www.w3.org/ns/odrl/2/Privacy" => PolicyUnion::Privacy(Privacy::default()),
           "http://www.w3.org/ns/odrl/2/Policy" |
           "http://www.w3.org/ns/odrl/2/Set" => PolicyUnion::Set(Set::default()),
           "http://www.w3.org/ns/odrl/2/Assert" => PolicyUnion::Assert(Assert::default()),
           "http://www.w3.org/ns/odrl/2/Offer" => PolicyUnion::Offer(Offer::default()),
           _ => { PolicyUnion::Set(Set::default())}
        }
    }
}