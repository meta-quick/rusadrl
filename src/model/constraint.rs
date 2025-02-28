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

pub enum Operator {
    eq,
    gt,
    gteq,
    hasPart,
    isA,
    isAllOf,
    ifAnyOf,
    ifNoneOf,
    ifPartOf,
    lt,
    lteq,
    neq,
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "eq" => Ok(Operator::eq),
            "gt" => Ok(Operator::gt),
            "gteq" => Ok(Operator::gteq),
            "hasPart" => Ok(Operator::hasPart),
            "isA" => Ok(Operator::isA),
            "isAllOf" => Ok(Operator::isAllOf),
            "ifAnyOf" => Ok(Operator::ifAnyOf),
            "ifNoneOf" => Ok(Operator::ifNoneOf),
            "ifPartOf" => Ok(Operator::ifPartOf),
            "lt" => Ok(Operator::lt),
            "lteq" => Ok(Operator::lteq),
            "neq" => Ok(Operator::neq),
            _ => Err(anyhow::anyhow!("Invalid operator: {}", value)),
        }
    }
}

pub enum LeftOperandClass {
    absolutePosition, 
    absoluteSize,
    absoluteSpatialPosition,
    absoluteTemporalPosition,
    count,
    dateTime,
    delayPeriod,
    deliveryChannel,
    device,
    elapsedTime,
    event,
    fileFormat,
    industry,
    language,
    media,
    meteredTime,
    payAmount,
    percentage,
    product,
    purpose,
    recipient,
    relativePosition,
    relativeSize,
    relativeSpatialPosition,
    relativeTemporalPosition,
    resolution,
    spatial,
    spatialCoordinates,
    system,
    systemDevice,
    timeInterval,
    unitOfCount,
    version,
    virtualLocation
}

impl TryFrom<&str> for LeftOperandClass {
    type Error = anyhow::Error;
    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let input = val.to_lowercase();
        match input.as_str() {
            absolutePosition=> Ok(LeftOperandClass::absolutePosition),
            absoluteSize => Ok(LeftOperandClass::absoluteSize),
            absoluteSpatialPosition => Ok(LeftOperandClass::absoluteSpatialPosition),
            absoluteTemporalPosition=> Ok(LeftOperandClass::absoluteTemporalPosition),
            count=> Ok(LeftOperandClass::count),
            dateTime=> Ok(LeftOperandClass::dateTime),
            delayPeriod=> Ok(LeftOperandClass::delayPeriod),
            deliveryChannel=> Ok(LeftOperandClass::deliveryChannel),
            device=> Ok(LeftOperandClass::device),
            elapsedTime=> Ok(LeftOperandClass::elapsedTime),
            event=> Ok(LeftOperandClass::event),
            fileFormat=> Ok(LeftOperandClass::fileFormat),
            industry=> Ok(LeftOperandClass::industry),
            language=> Ok(LeftOperandClass::language),
            media=> Ok(LeftOperandClass::media),
            meteredTime=> Ok(LeftOperandClass::meteredTime),
            payAmount=> Ok(LeftOperandClass::payAmount),
            percentage=> Ok(LeftOperandClass::percentage),
            product=> Ok(LeftOperandClass::product),
            purpose=> Ok(LeftOperandClass::purpose),
            recipient=> Ok(LeftOperandClass::recipient),
            relativePosition=> Ok(LeftOperandClass::relativePosition),
            relativeSize=> Ok(LeftOperandClass::relativeSize),
            relativeSpatialPosition=> Ok(LeftOperandClass::relativeSpatialPosition),
            relativeTemporalPosition=> Ok(LeftOperandClass::relativeTemporalPosition),
            resolution=> Ok(LeftOperandClass::resolution),
            spatial=> Ok(LeftOperandClass::spatial),
            spatialCoordinates=> Ok(LeftOperandClass::spatialCoordinates),
            system=> Ok(LeftOperandClass::system),
            systemDevice=> Ok(LeftOperandClass::systemDevice),
            timeInterval=> Ok(LeftOperandClass::timeInterval),
            unitOfCount=> Ok(LeftOperandClass::unitOfCount),
            version=> Ok(LeftOperandClass::version),
            virtualLocation=> Ok(LeftOperandClass::virtualLocation),
            _ => Err(anyhow::anyhow!("Invalid left operand class: {}", val)),
        }
    }
}

pub struct LeftOperand {
    pub class: LeftOperandClass,
    pub value: String,
}

pub enum RightOperand {
    Literal(String),
    Set(Vec<IriBuf>),
    RightOperandReference(IriBuf),
}

#[derive(Debug,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Constraint {
    pub uid: Option<IriBuf>,

    //Identifier: http://www.w3.org/ns/odrl/2/unit
    pub unit: String,
    //Identifier: http://www.w3.org/ns/odrl/2/status
    pub status: bool,
    //Identifier: http://www.w3.org/ns/odrl/2/dataType
    //JSONLD: @type
    pub dataType: String,
    //Identifier: http://www.w3.org/ns/odrl/2/Operator
    pub operator: String,

    pub leftOperand: LeftOperand,
    pub rightOperand: RightOperand,
    pub metadata: Metadata,
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
            operator: String::new(),
            leftOperand: String::new(),
            rightOperand: String::new(),
            metadata: Metadata::new(),
        }
    }
}


