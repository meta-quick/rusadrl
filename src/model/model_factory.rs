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