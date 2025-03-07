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

use lombok::{Builder, Getter, GetterMut, Setter};


#[derive(Debug,Clone)]
pub enum ConstraintOperator {
    //http://www.w3.org/ns/odrl/2/eq
    eq,
    //http://www.w3.org/ns/odrl/2/gt
    gt,
    //http://www.w3.org/ns/odrl/2/gteq
    gteq,
    //http://www.w3.org/ns/odrl/2/lt
    lt,
    //http://www.w3.org/ns/odrl/2/lteq
    lteq,
    //http://www.w3.org/ns/odrl/2/neq
    neq,
    //http://www.w3.org/ns/odrl/2/isA
    isA,
    //http://www.w3.org/ns/odrl/2/hastPart
    hasPart,
    //http://www.w3.org/ns/odrl/2/isPartOf
    isPartOf,
    //http://www.w3.org/ns/odrl/2/isAllOf
    isAllOf,
    //http://www.w3.org/ns/odrl/2/isAnyOf
    isAnyOf,
    //http://www.w3.org/ns/odrl/2/isNoneOf
    isNoneOf
}

impl TryFrom<&str> for ConstraintOperator {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            //please match all the operator in the order of the enum
            "eq" => Ok(ConstraintOperator::eq),
            "gt" => Ok(ConstraintOperator::gt),
            "gteq" => Ok(ConstraintOperator::gteq),
            "lt" => Ok(ConstraintOperator::lt),
            "lteq" => Ok(ConstraintOperator::lteq),
            "neq" => Ok(ConstraintOperator::neq),
            "isa" => Ok(ConstraintOperator::isA),
            "haspart" => Ok(ConstraintOperator::hasPart),
            "ispartof" => Ok(ConstraintOperator::isPartOf),
            "isallof" => Ok(ConstraintOperator::isAllOf),
            "isanyof" => Ok(ConstraintOperator::isAnyOf),
            "isnoneof" => Ok(ConstraintOperator::isNoneOf),
            _ => Err(anyhow::anyhow!("Invalid operator: {}", value)),
        }
    }
}   


pub enum ConstraintLogicOperator {
    //http://www.w3.org/ns/odrl/2/or
    or,
    //http://www.w3.org/ns/odrl/2/xone
    xone,
    //http://www.w3.org/ns/odrl/2/and
    and,
    //http://www.w3.org/ns/odrl/2/andSequence
    andSequence
}

impl TryFrom<&str> for ConstraintLogicOperator {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            //please match all the operator in the order of the enum
            "or" => Ok(ConstraintLogicOperator::or),
            "xone" => Ok(ConstraintLogicOperator::xone),
            "and" => Ok(ConstraintLogicOperator::and),
            "andsequence" => Ok(ConstraintLogicOperator::andSequence),
            _ => Err(anyhow::anyhow!("Invalid operator: {}", value)),
        }
    }
}