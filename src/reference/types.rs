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
#![allow(non_camel_case_types)]

use lombok::{Builder,Setter};

#[derive(Debug,Default, Clone)]
pub enum OperandValueType {
    #[default]
    string,
    set,
}

#[derive(Debug,Default,Builder,Setter, Clone)]
pub struct OperandValue {
    pub ty: OperandValueType,
    pub sets: Option<Vec<String>>,
    pub sval: Option<String>
}

impl OperandValue {
    pub fn get_sval(&self) -> Option<String> {
       self.sval.clone()
    }

    pub fn get_set(&self) -> Option<Vec<String>> {
        self.sets.clone()
    }
}