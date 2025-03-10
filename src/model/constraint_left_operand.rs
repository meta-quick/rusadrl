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

use anyhow::anyhow;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::stateworld::StateWorld;
use crate::traits::traits::OperandValue;

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

impl OperandValue for ConstraintLeftOperand {
    fn value(&self,world: &mut StateWorld) -> Result<Self,anyhow::Error> {
        match self {
            //please match all the operator in the order of the enum
            Self::absolutePosition => {
                world.get_state(Self::absolutePosition.into())
            },
            Self::absoluteSpatialPosition => {
                world.get_state(Self::absoluteSpatialPosition.into())
            },
            Self::absoluteTemporalPosition => {
                world.get_state(Self::absoluteTemporalPosition.into())
            },
            Self::absoluteSize => {
                world.get_state(Self::absoluteSize.into())
            },
            Self::count => {
                world.get_state(Self::count.into())
            },
            Self::datetime => {
                let now = world.now();
                Some(now.into())
            },
            Self::deliveryChannel => {
                Ok(Self::deliveryChannel.into())
            },
            Self::elapsedTime => {
                let elapsed = world.worldInitialTime;
                let now = world.now() - elapsed;
                Some(now.into())
            },
            Self::event => {
                Ok(Self::event.into())
            },
            Self::fileFormat => {
                Ok(Self::fileFormat.into())
            },
            Self::industry => {
                Ok(Self::industry.into())
            },
            Self::language => {
                Ok(Self::language.into())
            },
            Self::media => {
                Ok(Self::media.into())
            },
            Self::meteredTime => {
                Ok(Self::meteredTime.into())
            },
            Self::payAmount => {
                Ok(Self::payAmount.into())
            },
            Self::percentage => {
                Ok(Self::percentage.into())
            },
            Self::product => {
                Ok(Self::product.into())
            },
            Self::purpose => {
                Ok(Self::purpose.into())
            },
            Self::recipient => {
                Ok(Self::recipient.into())
            },
            Self::relativePosition => {
                Ok(Self::relativePosition.into())
            },
            Self::relativeSpatialPosition => {
                Ok(Self::relativeSpatialPosition.into())
            },
            Self::relativeTemporalPosition => {
                Ok(Self::relativeTemporalPosition.into())
            },
            Self::relativeSize => {
                Ok(Self::relativeSize.into())
            },
            Self::resolution => {
                Ok(Self::resolution.into())
            },
            Self::spatial => {
                Ok(Self::spatial.into())
            },
            Self::spatialCoordinates => {
                Ok(Self::spatialCoordinates.into())
            },
            Self::systemDevice => {
                Ok(Self::systemDevice.into())
            },
            Self::timeInterval => {
                let now = world.now();
                let last = world.last_time();
                let interval = now - last;
                Some(interval.into())
            },
            Self::unit => {
                Ok(Self::unit.into())
            },
            Self::version => {
                Ok(Self::version.into())
            },
            Self::virtualLocation => {
                Ok(Self::virtualLocation.into())
            },
            Self::delayPeriod => {
                Ok(Self::delayPeriod.into())
            },
            Self::deliveryChannel => {
                Ok(Self::deliveryChannel.into())
            },
            Self::elapsedTime => {
                Ok(Self::elapsedTime.into())
            },
            Self::event => {
                Ok(Self::event.into())
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