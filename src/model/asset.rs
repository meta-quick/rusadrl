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

use crate::model::metadata::Metadata;
use super::constraint::Constraint;

//http://www.w3.org/ns/odrl/2/AssetCollection
#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct AssetCollection {
    pub source  : Option<IriBuf>,
    pub refinement: Option<Vec<Constraint>>,
    pub metadata: Metadata,
}

//http://www.w3.org/ns/odrl/2/Asset
#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Asset {
    //unique identifier of the asset
    pub uid: Option<IriBuf>,
    //part of the asset collection
    pub partOf: Option<Vec<IriBuf>>,
    //refer to policy definition by IRI of Policy
    pub hasPolicy: Option<IriBuf>,
    //common metadata
    pub metadata: Option<Metadata>,
}


