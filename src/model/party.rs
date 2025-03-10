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

use iref::IriBuf;
use lombok::{Getter,Builder,Setter,GetterMut};

use crate::model::metadata::Metadata;

#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct PartyCollection {
    pub source: Option<IriBuf>,
    pub metadata: Metadata,
}

#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct Party {
    pub uid: Option<IriBuf>,
    pub partOf: Vec<IriBuf>,
    pub assignerOf: Option<IriBuf>,
    pub assigneeOf: Option<IriBuf>,
    pub metadata: Metadata,
}

impl Party {
    pub fn new() -> Self {
        Party {
            uid: None,
            metadata: Metadata::new(),
            partOf: Vec::new(),
            assignerOf: None,
            assigneeOf: None,
        }
    }
}