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
use crate::model::constraint_left_operand::ConstraintLeftOperand;
use crate::model::data_type::DataType;
use crate::reference::types::OperandValue;

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

impl ConstraintOperator {
    pub fn eval(&self,dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match self {
            ConstraintOperator::eq => {
               return self.eq(dty,left,right);
            },
            ConstraintOperator::gt => {
               return self.gt(dty,left,right);
            },
            ConstraintOperator::gteq => {
                return  self.gteq(dty,left,right);
            },
            ConstraintOperator::lt => {
                return self.lt(dty,left,right);
            },
            ConstraintOperator::lteq => {
                return self.lteq(dty,left,right);
            },
            ConstraintOperator::neq => {
                return self.neq(dty,left,right);
            },
            ConstraintOperator::isA => {
                return self.isA(dty,left,right);
            },
            ConstraintOperator::hasPart => {
                return self.hasPart(dty,left,right);
            },
            ConstraintOperator::isPartOf => {
                return self.isPartOf(dty,left,right);
            },
            ConstraintOperator::isAllOf => {
                return self.isAllOf(dty,left,right);
            },
            ConstraintOperator::isAnyOf => {
                return self.isAnyOf(dty,left,right);
            },
            ConstraintOperator::isNoneOf => {
                return self.isNoneOf(dty,left,right);
            },
            _ => {
                return false;
            }
        }
        true
    }

    fn gt(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
           DataType::Integer => {
               let left = left.get_sval().unwrap();
               let right = right.get_sval().unwrap();
               let left = left.parse::<i64>().unwrap();
               let right = right.parse::<i64>().unwrap();
               left == right
           },
           DataType::Float => {
               let left = left.get_sval().unwrap();
               let right = right.get_sval().unwrap();
               let left = left.parse::<f64>().unwrap();
               let right = right.parse::<f64>().unwrap();
               if (left - right).abs() < std::f64::EPSILON {
                   return true;
               }
               false
           },
           DataType::Date => {
               let left = left.get_sval().unwrap();
               let right = right.get_sval().unwrap();
               let left = left.parse::<i64>().unwrap();
               let right = right.parse::<i64>().unwrap();
               left == right
           },
           DataType::Time => {
               let left = left.get_sval().unwrap();
               let right = right.get_sval().unwrap();
               let left = left.parse::<i64>().unwrap();
               let right = right.parse::<i64>().unwrap();
               left == right
           }
           _ => false
        }
    }

    fn eq(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                left == right
            },
            DataType::Integer => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left == right
            },
            DataType::Float => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<f64>().unwrap();
                let right = right.parse::<f64>().unwrap();
                if (left - right).abs() < f64::EPSILON {
                    return true;
                }
                false
            },
            DataType::Boolean => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<bool>().unwrap();
                let right = right.parse::<bool>().unwrap();
                left == right
            },
            DataType::Date |
            DataType::DateTime |
            DataType::Time => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left == right
            }
        }
    }

    fn gteq(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::Integer => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left >= right
            },
            DataType::Float => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<f64>().unwrap();
                let right = right.parse::<f64>().unwrap();

                left - right >= f64::EPSILON
            },
            DataType::Date => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left >= right
            },
            DataType::Time => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left >= right
            }
            _ => {
                false
            }
        }
    }
    fn lt(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::Integer => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left < right
            }
            DataType::Float => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<f64>().unwrap();
                let right = right.parse::<f64>().unwrap();
                left - right < f64::EPSILON
            },
            DataType::Date => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left < right
            }
            DataType::Time => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left < right
            }
            _ => {
                false
            }
        }
    }
    fn lteq(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::Integer => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left <= right
            },
            DataType::Float => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<f64>().unwrap();
                let right = right.parse::<f64>().unwrap();
                left - right <= f64::EPSILON
            }
            DataType::Date => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left <= right
            }
            DataType::Time => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left <= right
            }
            _ => {
                false
            }
        }
    }

    fn neq(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                left != right
            }
            DataType::Integer => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left != right
            },
            DataType::Float => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<f64>().unwrap();
                let right = right.parse::<f64>().unwrap();
                (left - right) > f64::EPSILON
            }
            DataType::Date => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left != right
            }
            DataType::Time => {
                let left = left.get_sval().unwrap();
                let right = right.get_sval().unwrap();
                let left = left.parse::<i64>().unwrap();
                let right = right.parse::<i64>().unwrap();
                left != right
            }
            _ => {
                false
            }
        }
    }

    fn isA(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_sval().unwrap();
                let right = right.get_set().unwrap();
                right.contains(&left)
            },
            _ => {
                false
            }
        }
    }

    fn hasPart(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_sval().unwrap();
                let right = right.get_set().unwrap();
                right.contains(&left)
            },
            _ => {
                false
            }
        }
    }

    fn isPartOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_sval().unwrap();
                let right = right.get_set().unwrap();
                right.contains(&left)
            }
            _ => {
                false
            }
        }
    }

    fn isAllOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_set().unwrap();
                let right = right.get_set().unwrap();
                left.iter().all(|x| right.contains(x))
            }
            _ => {
                false
            }
        }
    }

    fn isAnyOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_set().unwrap();
                let right = right.get_set().unwrap();
                left.iter().any(|x| right.contains(x))
            }
            _ => {
                false
            }
        }
    }
    fn isNoneOf(&self, dty: DataType, left: &OperandValue, right: &OperandValue) -> bool {
        match dty {
            DataType::String
            | DataType::Integer
            | DataType::Float
            | DataType::Date
            | DataType::Time
            => {
                let left = left.get_set().unwrap();
                let right = right.get_set().unwrap();
                let result = left.iter().any(|x| right.contains(x));
                !result
            }
            _ => {
                false
            }
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