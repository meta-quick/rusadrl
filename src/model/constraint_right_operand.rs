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
use lombok::{Builder, Getter, GetterMut, Setter};

#[derive(Debug,Default, Clone)]
pub enum RightOperandType {
    #[default]
    Literal,
    LiteralSet,
    Reference
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct RightOperandReference {
    pub reference: Option<IriBuf>
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct ConstraintRightOperand {
    pub ty: RightOperandType,
    pub value: Option<String>,
    pub values: Option<Vec<String>>,
    pub reference: Option<RightOperandReference>
}

impl Default for RightOperandType {
    fn default() -> Self {
        RightOperandType::Literal
    }
}