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

use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};

use super::rule::Rule;

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Duty {
    pub rule: Rule,
    // pub consequence: IriBuf,
}