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

use std::f32::consts::E;
use std::str::FromStr;
use anyhow::anyhow;
use chrono::Duration;
use lombok::{Builder, Getter, GetterMut, Setter};
use serde_json::to_string;
use thiserror::Error;
use crate::model::stateworld::StateWorld;
use crate::reference::types::{OperandValue, OperandValueType};

// 自定义错误类型
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("持续时间字符串中包含无效字符")]
    InvalidCharacter,
    #[error("持续时间字符串组件不完整")]
    IncompleteComponent,
    #[error("解析数字失败: {0}")]
    ParseNumberError(#[from] std::num::ParseFloatError),
}

// 解析 XML 持续时间字符串
pub fn parse_xml_duration(duration_str: &str) -> Result<Duration, ParseError> {
    let mut total_seconds: f64 = 0.0; // 使用 f64 以支持小数
    let mut current_num = String::new(); // 累积当前数字
    let mut is_negative = false; // 是否为负持续时间
    let mut in_time_part = false; // 是否在时间部分

    let mut chars = duration_str.chars().peekable();

    // 检查负号
    if let Some('-') = chars.peek() {
        is_negative = true;
        chars.next();
    }

    // 确保以 'P' 开头
    if chars.next() != Some('P') {
        return Err(ParseError::InvalidCharacter);
    }

    // 逐字符解析
    while let Some(ch) = chars.next() {
        match ch {
            'T' => {
                in_time_part = true;
                continue;
            }
            'Y' | 'M' | 'D' | 'H' | 'S' => {
                if current_num.is_empty() {
                    return Err(ParseError::IncompleteComponent);
                }
                let value = f64::from_str(&current_num)?;
                current_num.clear();
                total_seconds += match (ch, in_time_part) {
                    ('Y', false) => value * 365.25 * 24.0 * 3600.0, // 年，考虑闰年
                    ('M', false) => value * 30.44 * 24.0 * 3600.0,  // 月，平均长度
                    ('D', false) => value * 24.0 * 3600.0,          // 天
                    ('H', true) => value * 3600.0,                  // 小时
                    ('M', true) => value * 60.0,                    // 分钟
                    ('S', true) => value,                           // 秒
                    _ => return Err(ParseError::InvalidCharacter),
                };
            }
            c if c.is_digit(10) || c == '.' => current_num.push(c),
            _ => return Err(ParseError::InvalidCharacter),
        }
    }

    // 检查是否有未处理的数字
    if !current_num.is_empty() {
        return Err(ParseError::IncompleteComponent);
    }

    // 应用正负号
    let total_seconds = if is_negative {
        -total_seconds
    } else {
        total_seconds
    };

    Ok(Duration::seconds(total_seconds as i64))
}


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
            | ConstraintLeftOperand::delayPeriod
            => {
                let state = self.to_iri().unwrap();
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
            ConstraintLeftOperand::timeInterval => {
                let time_interval = world.timeInterval();
                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                val.set_sval(Some(time_interval.to_string()));
                return Ok(val);
            }
            ConstraintLeftOperand::meteredTime => {
                let metered_time = world.meteredTime();
                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                val.set_sval(Some(metered_time.to_string()));
                return Ok(val);
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
        }
    }

    pub fn to_iri(&self) -> Result<String, anyhow::Error> {
        match self {
            ConstraintLeftOperand::absolutePosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/absolutePosition"))
            }
            ConstraintLeftOperand::absoluteSpatialPosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/absoluteSpatialPosition"))
            }
            ConstraintLeftOperand::absoluteTemporalPosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/absoluteTemporalPosition"))
            }
            ConstraintLeftOperand::absoluteSize => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/absoluteSize"))
            }
            ConstraintLeftOperand::count => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/count"))
            }
            ConstraintLeftOperand::datetime => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/datetime"))
            }
            ConstraintLeftOperand::delayPeriod => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/delayPeriod"))
            }
            ConstraintLeftOperand::deliveryChannel => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/deliveryChannel"))
            }
            ConstraintLeftOperand::elapsedTime => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/elapsedTime"))
            }
            ConstraintLeftOperand::event => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/event"))
            }
            ConstraintLeftOperand::fileFormat => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/fileFormat"))
            }
            ConstraintLeftOperand::industry => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/industry"))
            }
            ConstraintLeftOperand::language => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/language"))
            }
            ConstraintLeftOperand::media => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/media"))
            }
            ConstraintLeftOperand::meteredTime => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/meteredTime"))
            }
            ConstraintLeftOperand::payAmount => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/payAmount"))
            }
            ConstraintLeftOperand::percentage => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/percentage"))
            }
            ConstraintLeftOperand::product => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/product"))
            }
            ConstraintLeftOperand::purpose => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/purpose"))
            }
            ConstraintLeftOperand::recipient => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/recipient"))
            }
            ConstraintLeftOperand::relativePosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/relativePosition"))
            }
            ConstraintLeftOperand::relativeSpatialPosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/relativeSpatialPosition"))
            }
            ConstraintLeftOperand::relativeTemporalPosition => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/relativeTemporalPosition"))
            }
            ConstraintLeftOperand::relativeSize => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/relativeSize"))
            }
            ConstraintLeftOperand::resolution => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/resolution"))
            }
            ConstraintLeftOperand::spatial => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/spatial"))
            }
            ConstraintLeftOperand::spatialCoordinates => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/spatialCoordinates"))
            }
            ConstraintLeftOperand::systemDevice => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/systemDevice"))
            }
            ConstraintLeftOperand::timeInterval => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/timeInterval"))
            }
            ConstraintLeftOperand::unit => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/unit"))
            }
            ConstraintLeftOperand::version => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/version"))
            }
            ConstraintLeftOperand::virtualLocation => {
                Ok(String::from("http://www.w3.org/ns/odrl/2/virtualLocation"))
            }
        }
    }
}

impl TryFrom<&str> for ConstraintLeftOperand {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.to_lowercase();
        if value.contains("/") {
            let index = value.rfind("/").unwrap();
            value = value.split_at(index+1).1.to_string();
        }


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

    #[test]
    fn test_constraint_left_operand_error() {
        let duration_str = "P1Y2M3DT4H5M6S"; // Example XML duration string
        match parse_xml_duration(duration_str) {
            Ok(duration) => println!("Total duration in seconds: {}", duration.num_seconds()),
            Err(e) => println!("Error parsing duration: {}", e),
        }
    }
}
