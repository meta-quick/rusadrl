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
#![warn(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

//http://www.w3.org/ns/odrl/2/ConflictTerm
#[derive(Debug,Clone,PartialEq,PartialOrd)]
pub enum ConflictStrategy {
    //http://www.w3.org/ns/odrl/2/prohibit
    prohibit,
    //http://www.w3.org/ns/odrl/2/perm
    perm,
    //http://www.w3.org/ns/odrl/2/invalid
    invalid,
}

impl TryFrom<&str> for ConflictStrategy {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "prohibit" => Ok(ConflictStrategy::prohibit),
            "perm" => Ok(ConflictStrategy::perm),
            "invalid" => Ok(ConflictStrategy::invalid),
            _ => Err(anyhow::anyhow!("invalid conflict strategy: {}", value)),
        }
    }
}