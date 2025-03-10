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

#[derive(Debug,Default,PartialEq, Eq, Clone)]
pub enum PolicyClassType {
    SET,
    OFFER,
    AGREEMENT,
    #[default]
    NONE,
}

impl TryFrom<String> for PolicyClassType {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "set" => Ok(PolicyClassType::SET),
            "offer" => Ok(PolicyClassType::OFFER),
            "aggrement" => Ok(PolicyClassType::AGREEMENT),
            _ => Ok(PolicyClassType::NONE),
        }
    }
}
