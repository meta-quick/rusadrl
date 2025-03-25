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
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use std::f32::consts::E;
use std::str::FromStr;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::data_type::DataType;
use crate::reference::types::OperandValue;
use chrono::{NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use crate::model::{constraint_left_operand::parse_xml_duration };


#[derive(Debug,Clone)]
#[allow(non_camel_case_types)]
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

fn parse_datetime(input: &str) -> i64 {
    let date = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S");
    if let Ok(date) = date {
        return Utc.from_utc_datetime(&date).timestamp_millis()
    } else {
        //try to parse date
        let date =  NaiveDate::parse_from_str(input, "%Y-%m-%d");
        if let Ok(date) = date {
            let datetime = date.and_hms(0, 0, 0);
            return Utc.from_utc_datetime(&datetime).timestamp_millis()
        }else {
            //try to parse time
            let time = chrono::NaiveTime::parse_from_str(input, "%H:%M:%S");
            if let Ok(time) = time {
                let seconds = time.num_seconds_from_midnight() as i64;
                let milliseconds = seconds * 1_000;
                return milliseconds;
            } else {
                let date =  parse_xml_duration(input);
                if let Ok(date) = date {
                    return date.num_milliseconds();
                } else {
                    return 0;
                }
            }
        }
    }
}

impl TryFrom<&str> for ConstraintOperator {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.to_lowercase();
        if value.contains("/") {
            let index = value.rfind("/").unwrap();
            value = value.split_at(index+1).1.to_string();
        }

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

impl ConstraintOperator {
    pub fn eval(&self,dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> bool {
        match self {
            ConstraintOperator::eq => {
                let result = self.eq(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::gt => {
                let result =  self.gt(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::gteq => {
                let result =  self.gteq(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::lt => {
                let result =  self.lt(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::lteq => {
                let result =  self.lteq(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::neq => {
                let result =  self.neq(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::isA => {
                let result =  self.isA(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::hasPart => {
                let result =  self.hasPart(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::isPartOf => {
                let result =  self.isPartOf(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::isAllOf => {
                let result =  self.isAllOf(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::isAnyOf => {
                let result =  self.isAnyOf(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            },
            ConstraintOperator::isNoneOf => {
                let result =  self.isNoneOf(dty,left,right,de);
                if let Ok(result) = result {
                    return result;
                }
                return false;
            }
        }
    }

    fn gt(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
           DataType::Integer => {
               let mut  left = left.get_sval();
               if left.is_none() {
                   //try to use de
                   if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                   }
                   left = de.clone();
               }
               let left = left.unwrap();

               let right = right.get_sval();
               if right.is_none() {
                   return Err(anyhow::anyhow!("Right operand is None"));
               }
               let right = right.unwrap();

               let left_val = left.parse::<i64>();
               if left_val.is_err() {
                   return Err(anyhow::anyhow!("Left operand has invalid value"));
               }
               let left_val = left_val.unwrap();

               //try to parse right as integer
               let right_val = right.parse::<i64>();
               if let Ok(right_val) = right_val {
                   return Ok(left_val > right_val);
               }

               //if right is not integer, treat it as a string
               let right = parse_xml_duration(right.as_str());
               if let Ok(right_val) = right {
                   return Ok(left_val > right_val.num_milliseconds());
               }
               return Ok(false);
           },
           DataType::Float => {
               let mut  left = left.get_sval();
               if left.is_none() {
                   //try to use de
                   if de.is_none() {
                       return Err(anyhow::anyhow!("Left operand is None"));
                   }
                   left = de.clone();
               }
               let left = left.unwrap();

               let right = right.get_sval();
               if right.is_none() {
                   return Err(anyhow::anyhow!("Right operand is None"));
               }
               let right = right.unwrap();


               let left_val = left.parse::<f64>();
               if left_val.is_err() {
                   return Err(anyhow::anyhow!("Left operand has invalid value"));
               }
               let left_val = left_val.unwrap();

               let right_val = right.parse::<f64>();
               if right_val.is_err() {
                   return Err(anyhow::anyhow!("Right operand has invalid value"));
               }
               let right_val = right_val.unwrap();

               if left_val - right_val > std::f64::EPSILON {
                   return Ok(true);
               }
               Ok(false)
           },
           DataType::Date => {
               let mut  left = left.get_sval();
               if left.is_none() {
                   //try to use de
                   if de.is_none() {
                       return Err(anyhow::anyhow!("Left operand is None"));
                   }
                   left = de.clone();
               }
               let left = left.unwrap();

               let right = right.get_sval();
               if right.is_none() {
                   return Err(anyhow::anyhow!("Right operand is None"));
               }
               let right = right.unwrap();

               let left_val = left.parse::<i64>();
               if left_val.is_err() {
                   return Err(anyhow::anyhow!("Left operand has invalid value"));
               }
               let left_val = left_val.unwrap();

               let right_val = parse_datetime(right.as_str());
               Ok(left_val > right_val)
           },
           DataType::DateTime |
           DataType::Time => {
               let mut  left = left.get_sval();
               if left.is_none() {
                   //try to use de
                   if de.is_none() {
                       return Err(anyhow::anyhow!("Left operand is None"));
                   }
                   left = de.clone();
               }
               let left = left.unwrap();

               let right = right.get_sval();
               if right.is_none() {
                   return Err(anyhow::anyhow!("Right operand is None"));
               }
               let right = right.unwrap();


               let left_val = left.parse::<i64>();
               if left_val.is_err() {
                   return Err(anyhow::anyhow!("Left operand has invalid value"));
               }
               let left_val = left_val.unwrap();

               let right_val = parse_datetime(right.as_str());
               Ok(left_val > right_val)
           }
           _ => Err(anyhow::anyhow!("Invalid operator"))
        }
    }

    fn eq(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(left == right)
            },
            DataType::Integer => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();


                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<i64>();
                if let Ok(right_val) = right_val {
                    return Ok(left_val == right_val);
                }

                //if right is not integer, treat it as a string
                let right = parse_xml_duration(right.as_str());
                if let Ok(right_val) = right {
                    return Ok(left_val == right_val.num_milliseconds());
                }
                return Err(anyhow::anyhow!("Right operand has invalid value"));
            },
            DataType::Float => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<f64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right = right.parse::<f64>();
                if let Ok(right_val) = right {
                    return Ok((left_val - right_val).abs() < f64::EPSILON);
                }
                return Err(anyhow::anyhow!("Right operand has invalid value"));
            },
            DataType::Boolean => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();


                let left_val = left.parse::<bool>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right = right.parse::<bool>();
                if right.is_err() {
                    return Err(anyhow::anyhow!("Right operand has invalid value"));
                }
                let right_val = right.unwrap();

                Ok(left_val == right_val)
            },
            DataType::Date |
            DataType::DateTime |
            DataType::Time => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val == right_val)
            }
        }
    }

    fn gteq(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de :&Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::Integer => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<i64>();
                if let Ok(right_val) = right_val {
                    return Ok(left_val >= right_val);
                }
                //if right is not integer, treat it as a string
                let right = parse_xml_duration(right.as_str());
                if let Ok(right_val) = right {
                    return Ok(left_val >= right_val.num_milliseconds());
                }
                return Err(anyhow::anyhow!("Right operand has invalid value"));
            },
            DataType::Float => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();


                let left_val = left.parse::<f64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<f64>();
                if right_val.is_err() {
                    return Err(anyhow::anyhow!("Right operand has invalid value"));
                }
                let right_val = right_val.unwrap();


                Ok(left_val - right_val >= f64::EPSILON)
            },
            DataType::Date => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val >= right_val)
            },
            DataType::DateTime |
            DataType::Time => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val >= right_val)
            }
            _ => {
                Result::Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }
    fn lt(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error>  {
        match dty {
            DataType::Integer => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<i64>();
                if let Ok(right_val) = right_val {
                    return Ok(left_val < right_val);
                }

                //if right is not integer, treat it as a string
                let right = parse_xml_duration(right.as_str());
                if let Ok(right_val) = right {
                    return Ok(left_val < right_val.num_milliseconds());
                }
                return Err(anyhow::anyhow!("Right operand has invalid value"));
            }
            DataType::Float => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<f64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right = right.parse::<f64>();
                if right.is_err() {
                    return Err(anyhow::anyhow!("Right operand has invalid value"));
                }
                let right_val = right.unwrap();
                Ok(left_val - right_val < f64::EPSILON)
            },
            DataType::Date => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val < right_val)
            }
            DataType::Time => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val < right_val)
            }
            DataType::DateTime => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());

                Ok(left_val < right_val)
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }
    fn lteq(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::Integer => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<i64>();
                if let Ok(right_val) = right_val {
                    return Ok(left_val <= right_val);
                }

                //if right is not integer, treat it as a string
                let right = parse_xml_duration(right.as_str());
                if let Ok(right_val) = right {
                    return Ok(left_val <= right_val.num_milliseconds());
                }
                return Err(anyhow::anyhow!("Right operand has invalid value"));
            },
            DataType::Float => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<f64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<f64>();
                if right_val.is_err() {
                    return Err(anyhow::anyhow!("Right operand has invalid value"));
                }
                let right_val = right_val.unwrap();
                Ok(left_val - right_val <= f64::EPSILON)
            }
            DataType::Date => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left = left.parse::<i64>();
                if left.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val <= right_val)
            }
            DataType::Time => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left = left.parse::<i64>();
                if left.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val <= right_val)
            }
            DataType::DateTime => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val <= right_val)
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    fn neq(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(left != right)
            }
            DataType::Integer => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = right.parse::<i64>();
                if let Ok(right_val) = right_val {
                    return Ok(left_val != right_val);
                }
                //if right is not integer, treat it as a string
                let right = parse_xml_duration(right.as_str());
                if let Ok(right_val) = right {
                    return Ok(left_val != right_val.num_milliseconds());
                }
                return Ok(false);
            },
            DataType::Float => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<f64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();
                let right_val = right.parse::<f64>();
                if right_val.is_err() {
                    return Err(anyhow::anyhow!("Right operand has invalid value"));
                }
                let right_val = right_val.unwrap();

                Ok((left_val - right_val) > f64::EPSILON)
            }
            DataType::Date => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val != right_val)
            }
            DataType::Time => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val != right_val)
            }
            DataType::DateTime => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_sval();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let left_val = left.parse::<i64>();
                if left_val.is_err() {
                    return Err(anyhow::anyhow!("Left operand has invalid value"));
                }
                let left_val = left_val.unwrap();

                let right_val = parse_datetime(right.as_str());
                Ok(left_val != right_val)
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    #[allow(non_snake_case)]
    fn isA(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(right.contains(&left))
            },
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    #[allow(non_snake_case)]
    fn hasPart(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut  left = left.get_sval();
                if left.is_none() {
                    //try to use de
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(right.contains(&left))
            },
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    fn isPartOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut left = left.get_sval();
                if left.is_none() {
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = de.clone();
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(right.contains(&left))
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    fn isAllOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut left = left.get_set();
                if left.is_none() {
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = Some(vec![de.clone().unwrap()]);
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(left.iter().all(|x| right.contains(x)))
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }

    fn isAnyOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut left = left.get_set();
                if left.is_none() {
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = Some(vec![de.clone().unwrap()]);
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                Ok(left.iter().any(|x| right.contains(x)))
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }
    fn isNoneOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue,de: &Option<String>) -> Result<bool, anyhow::Error> {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let mut left = left.get_set();
                if left.is_none() {
                    if de.is_none() {
                        return Err(anyhow::anyhow!("Left operand is None"));
                    }
                    left = Some(vec![de.clone().unwrap()]);
                }
                let left = left.unwrap();

                let right = right.get_set();
                if right.is_none() {
                    return Err(anyhow::anyhow!("Right operand is None"));
                }
                let right = right.unwrap();

                let result = left.iter().any(|x| right.contains(x));
                Ok(!result)
            }
            _ => {
                Err(anyhow::anyhow!("Invalid operator"))
            }
        }
    }
}

#[derive(Debug,Clone)]
#[allow(non_camel_case_types)]
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
        //find the last /
        let index = value.rfind("/").unwrap();
        let value = value.split_at(index+1).1.to_string();
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use crate::model::constraint_operator::parse_datetime;

    #[test]
    fn test_parse_operator() {
        let now = chrono::Utc::now().timestamp_millis();

        let date = NaiveDate::parse_from_str("2025-3-24 00:00:00", "%Y-%m-%d %H:%M:%S");
        let date = date.unwrap().and_hms(0,0,0).timestamp_millis();

        println!("{:?}", date);
        println!("{:?}", now);
        println!("{:?}", now - date);
    }

    #[test]
    fn test_parse_datetime() {
        let t1 = parse_datetime("2025-3-24 00:00:00");
        let t2 = parse_datetime("2025-3-24");
        let t3 = parse_datetime("10:00:00");
        let t4 = parse_datetime("11:00:00");
        let t5 = parse_datetime("PT1H");

        //print each time
        println!("{:?}", t1);
        println!("{:?}", t2);
        println!("{:?}", t3);
        println!("{:?}", t4);
        println!("{:?}", t5);
    }
}