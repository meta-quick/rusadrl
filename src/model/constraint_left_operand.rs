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

use std::str::FromStr;
use anyhow::anyhow;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::stateworld::StateWorld;
use crate::reference::types::{OperandValue, OperandValueType};

#[derive(Debug,Clone)]
pub enum ConstraintLeftOperand {
    //http://www.w3.org/ns/odrl/2/absolutePosition
    absolutePosition,
    //http://www.w3.org/ns/odrl/2/absoluteSpatialPosition
    absoluteSpatialPosition,
    //http://www.w3.org/ns/odrl/2/absoluteTemporalPosition
    absoluteTemporalPosition,
    //http://www.w3.org/ns/odrl/2/absoluteSize
    absoluteSize,
    //http://www.w3.org/ns/odrl/2/count
    count,
    //http://www.w3.org/ns/odrl/2/datetime
    datetime,
    //http://www.w3.org/ns/odrl/2/delayPeriod
    delayPeriod,
    //http://www.w3.org/ns/odrl/2/deliveryChannel
    deliveryChannel,
    //http://www.w3.org/ns/odrl/2/elapsedTime
    elapsedTime,
    //http://www.w3.org/ns/odrl/2/event
    event,
    //http://www.w3.org/ns/odrl/2/fileFormat
    fileFormat,
    //http://www.w3.org/ns/odrl/2/industry
    industry,
    //http://www.w3.org/ns/odrl/2/language
    language,
    //http://www.w3.org/ns/odrl/2/media
    media,
    //http://www.w3.org/ns/odrl/2/meteredTime
    meteredTime,
    //http://www.w3.org/ns/odrl/2/payAmount
    payAmount,
    //http://www.w3.org/ns/odrl/2/percentage
    percentage,
    //http://www.w3.org/ns/odrl/2/product
    product,
    //http://www.w3.org/ns/odrl/2/purpose
    purpose,
    //http://www.w3.org/ns/odrl/2/recipient
    recipient,
    //http://www.w3.org/ns/odrl/2/relativePosition
    relativePosition,
    //http://www.w3.org/ns/odrl/2/relativeSpatialPosition
    relativeSpatialPosition,
    //http://www.w3.org/ns/odrl/2/relativeTemporalPosition
    relativeTemporalPosition,
    //http://www.w3.org/ns/odrl/2/relativeSize
    relativeSize,
    //http://www.w3.org/ns/odrl/2/resolution
    resolution,
    //http://www.w3.org/ns/odrl/2/spatial
    spatial,
    //http://www.w3.org/ns/odrl/2/spatialCoordinates
    spatialCoordinates,
    //http://www.w3.org/ns/odrl/2/systemDevice
    systemDevice,
    //http://www.w3.org/ns/odrl/2/timeInterval
    timeInterval,
    //http://www.w3.org/ns/odrl/2/unit
    unit,
    //http://www.w3.org/ns/odrl/2/version
    version,
    //http://www.w3.org/ns/odrl/2/virtualLocation
    virtualLocation
}

impl  ConstraintLeftOperand {
    pub fn value(&self, world: &mut StateWorld) -> Result<OperandValue,anyhow::Error> {
        match self {
            //please match all the operator in the order of the enum
            ConstraintLeftOperand::absolutePosition
            | ConstraintLeftOperand::absoluteSpatialPosition
            | ConstraintLeftOperand::absoluteTemporalPosition
            | ConstraintLeftOperand::absoluteSize
            | ConstraintLeftOperand::count
            | ConstraintLeftOperand::deliveryChannel
            | ConstraintLeftOperand::event
            | ConstraintLeftOperand::fileFormat
            | ConstraintLeftOperand::industry
            | ConstraintLeftOperand::language
            | ConstraintLeftOperand::media
            | ConstraintLeftOperand::payAmount
            | ConstraintLeftOperand::percentage
            | ConstraintLeftOperand::product
            | ConstraintLeftOperand::purpose
            | ConstraintLeftOperand::recipient
            | ConstraintLeftOperand::relativePosition
            | ConstraintLeftOperand::relativeSpatialPosition
            | ConstraintLeftOperand::relativeTemporalPosition
            | ConstraintLeftOperand::relativeSize
            | ConstraintLeftOperand::resolution
            | ConstraintLeftOperand::spatial
            | ConstraintLeftOperand::spatialCoordinates
            | ConstraintLeftOperand::systemDevice
            | ConstraintLeftOperand::unit
            | ConstraintLeftOperand::version
            | ConstraintLeftOperand::virtualLocation
            | ConstraintLeftOperand::timeInterval
            | ConstraintLeftOperand::delayPeriod
            | ConstraintLeftOperand::meteredTime
            => {
                let state = String::try_from(self.clone()).unwrap();
                let state = world.get_state(state.as_str());

                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                match state {
                    Some(state) => {
                        val.set_sval(Some(state.to_owned()));
                        return Ok(val);
                    },
                    None => {
                        return Err(anyhow!("constraint left operand: {} not found",state.unwrap()));
                    }
                }
            }
            ConstraintLeftOperand::datetime => {
                let now = world.now();

                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                val.set_sval(Some(now.to_string()));
                Ok(val)
            },
            ConstraintLeftOperand::elapsedTime => {
                let eclipsed = world.eclipse_datetime();
                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                val.set_sval(Some(eclipsed.to_string()));

                Ok(val)
            }
            _ => {
                Err(anyhow!("Not supported yet!"))
            }
        }
    }
}

impl TryFrom<&str> for ConstraintLeftOperand {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            //please match all the operator in the order of the enum
            "absoluteposition" => Ok(ConstraintLeftOperand::absolutePosition),
            "absolutespatialposition" => Ok(ConstraintLeftOperand::absoluteSpatialPosition),
            "absolutetemporalposition" => Ok(ConstraintLeftOperand::absoluteTemporalPosition),
            "absolutesize" => Ok(ConstraintLeftOperand::absoluteSize),
            "count" => Ok(ConstraintLeftOperand::count),
            "datetime" => Ok(ConstraintLeftOperand::datetime),
            "delayperiod" => Ok(ConstraintLeftOperand::delayPeriod),
            "deliverychannel" => Ok(ConstraintLeftOperand::deliveryChannel),
            "elapsedtime" => Ok(ConstraintLeftOperand::elapsedTime),
            "event" => Ok(ConstraintLeftOperand::event),
            "fileformat" => Ok(ConstraintLeftOperand::fileFormat),
            "industry" => Ok(ConstraintLeftOperand::industry),
            "language" => Ok(ConstraintLeftOperand::language),
            "media" => Ok(ConstraintLeftOperand::media),
            "meteredtime" => Ok(ConstraintLeftOperand::meteredTime),
            "payamount" => Ok(ConstraintLeftOperand::payAmount),
            "percentage" => Ok(ConstraintLeftOperand::percentage),
            "product" => Ok(ConstraintLeftOperand::product),
            "purpose" => Ok(ConstraintLeftOperand::purpose),
            "recipient" => Ok(ConstraintLeftOperand::recipient),
            "relativeposition" => Ok(ConstraintLeftOperand::relativePosition),
            "relativespatialposition" => Ok(ConstraintLeftOperand::relativeSpatialPosition),
            "relativetemporalposition" => Ok(ConstraintLeftOperand::relativeTemporalPosition),
            "relativesize" => Ok(ConstraintLeftOperand::relativeSize),
            "resolution" => Ok(ConstraintLeftOperand::resolution),
            "spatial" => Ok(ConstraintLeftOperand::spatial),
            "spatialcoordinates" => Ok(ConstraintLeftOperand::spatialCoordinates),
            "systemdevice" => Ok(ConstraintLeftOperand::systemDevice),
            "timeinterval" => Ok(ConstraintLeftOperand::timeInterval),
            "unit" => Ok(ConstraintLeftOperand::unit),
            "version" => Ok(ConstraintLeftOperand::version),
            "virtuallocation" => Ok(ConstraintLeftOperand::virtualLocation),
            _ => Err(anyhow::anyhow!("Invalid operator: {}", value))
        }
    }
}

impl TryFrom<ConstraintLeftOperand> for String {
    type Error = anyhow::Error;

    fn try_from(value: ConstraintLeftOperand) -> Result<Self, Self::Error> {
        match value {
            ConstraintLeftOperand::absolutePosition => Ok("absolutePosition".to_string()),
            ConstraintLeftOperand::absoluteSpatialPosition => Ok("absoluteSpatialPosition".to_string()),
            ConstraintLeftOperand::absoluteTemporalPosition => Ok("absoluteTemporalPosition".to_string()),
            ConstraintLeftOperand::absoluteSize => Ok("absoluteSize".to_string()),
            ConstraintLeftOperand::count => Ok("count".to_string()),
            ConstraintLeftOperand::datetime => Ok("datetime".to_string()),
            ConstraintLeftOperand::delayPeriod => Ok("delayPeriod".to_string()),
            ConstraintLeftOperand::deliveryChannel => Ok("deliveryChannel".to_string()),
            ConstraintLeftOperand::elapsedTime => Ok("elapsedTime".to_string()),
            ConstraintLeftOperand::event => Ok("event".to_string()),
            ConstraintLeftOperand::fileFormat => Ok("fileFormat".to_string()),
            ConstraintLeftOperand::industry => Ok("industry".to_string()),
            ConstraintLeftOperand::language => Ok("language".to_string()),
            ConstraintLeftOperand::media => Ok("media".to_string()),
            ConstraintLeftOperand::meteredTime => Ok("meteredTime".to_string()),
            ConstraintLeftOperand::payAmount => Ok("payAmount".to_string()),
            ConstraintLeftOperand::percentage => Ok("percentage".to_string()),
            ConstraintLeftOperand::product => Ok("product".to_string()),
            ConstraintLeftOperand::purpose => Ok("purpose".to_string()),
            ConstraintLeftOperand::recipient => Ok("recipient".to_string()),
            ConstraintLeftOperand::relativePosition => Ok("relativePosition".to_string()),
            ConstraintLeftOperand::relativeSpatialPosition => Ok("relativeSpatialPosition".to_string()),
            ConstraintLeftOperand::relativeTemporalPosition => Ok("relativeTemporalPosition".to_string()),
            ConstraintLeftOperand::relativeSize => Ok("relativeSize".to_string()),
            ConstraintLeftOperand::resolution => Ok("resolution".to_string()),
            ConstraintLeftOperand::spatial => Ok("spatial".to_string()),
            ConstraintLeftOperand::spatialCoordinates => Ok("spatialCoordinates".to_string()),
            ConstraintLeftOperand::systemDevice => Ok("systemDevice".to_string()),
            ConstraintLeftOperand::timeInterval => Ok("timeInterval".to_string()),
            ConstraintLeftOperand::unit => Ok("unit".to_string()),
            ConstraintLeftOperand::version => Ok("version".to_string()),
            ConstraintLeftOperand::virtualLocation => Ok("virtualLocation".to_string()),
        }
    }
}

impl FromStr for ConstraintLeftOperand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_constraint_left_operand() {
        let ver: ConstraintLeftOperand = "version".parse().unwrap();
        println!("{:?}", ver);
        let s: String= ver.try_into().unwrap();
        println!("{:?}", s);
    }
}
