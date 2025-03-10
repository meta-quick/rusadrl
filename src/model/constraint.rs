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

use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::metadata::Metadata;

use super::{constraint_left_operand::ConstraintLeftOperand, constraint_operator::ConstraintOperator, constraint_right_operand::ConstraintRightOperand};

//Identifier:	http://www.w3.org/ns/odrl/2/Constraint
#[derive(Debug,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Constraint {
    //Unique identifier of the constraint
    pub uid: Option<IriBuf>,

    //Identifier: http://www.w3.org/ns/odrl/2/unit
    pub unit: String,

    //Identifier: http://www.w3.org/ns/odrl/2/status
    pub status: bool,

    //Identifier: http://www.w3.org/ns/odrl/2/dataType
    //JSONLD: @type
    pub dataType: String,

    //Identifier: http://www.w3.org/ns/odrl/2/Operator
    pub operator: Option<ConstraintOperator>,

    pub leftOperand: Option<ConstraintLeftOperand>,
    pub rightOperand: Option<ConstraintRightOperand>,
    pub metadata: Option<Metadata>,
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint::new("http://www.w3.org/ns/odrl/2/Constraint")
    }
}

impl Constraint {
    /*
     * Identifier: 
     *   http://www.w3.org/ns/odrl/2/Constraint
     */
    pub fn new(iri: &str) -> Self {
        let uid = String::from(iri);
        Constraint {
            uid: Some(IriBuf::new(uid).unwrap()),
            unit: String::new(),
            status: false,
            dataType: String::new(),
            operator: None,
            leftOperand: None,
            rightOperand: None,
            metadata: None,
        }
    }
}


