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

use crate::model::metadata::Metadata;
use crate::model::constraint::Constraint;
use lombok::Builder;

#[derive(Debug, Default, Clone)]
pub struct PartyCollection {
    pub metadata: Metadata,
}

#[derive(Default, Builder， Debug, Clone)]
pub struct Party {
    pub metadata: Metadata,
    pub part_of: Vec<PartyCollection>,
    pub refinements: Vec<Constraint>,
}

impl Party {
    pub fn new() -> Self {
        Party {
            metadata: Metadata::new(),
            part_of: Vec::new(),
            refinements: Vec::new(),
        }
    }
}