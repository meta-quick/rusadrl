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
use crate::model::constraint_operator::ConstraintLogicOperator;
use crate::model::data_type::DataType;
use crate::model::metadata::Metadata;
use crate::model::stateworld::StateWorld;
use crate::traits::definions::LogicEval;
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

impl LogicEval for Constraint {
    fn eval(&self, mut world: &mut StateWorld) -> Result<bool, anyhow::Error> {
        if let None = self.operator {
            return Err(anyhow::Error::msg("No operator defined"));
        }

        if let None = self.leftOperand {
            return Err(anyhow::Error::msg("No left operand defined"));
        }

        let operator = self.operator.as_ref().unwrap();
        let left = self.leftOperand.as_ref().unwrap();
        let right = self.rightOperand.as_ref().unwrap();

        let left_value = left.value(&mut world).unwrap();
        let right_value = right.value(&mut world).unwrap();

        let dty = DataType::try_from(self.dataType.clone());
        match dty {
            Ok(dty) => {
                let result = operator.eval(dty,&left_value, &right_value);
                Ok(result)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}


#[derive(Debug,Builder, Clone)]
pub struct LogicConstraint {
    pub uid: Option<IriBuf>,
    pub operator: Option<ConstraintLogicOperator>,
    pub operand: Option<Vec<Constraint>>
}

impl Default for LogicConstraint {
    fn default() -> Self {
        LogicConstraint::new("http://www.w3.org/ns/odrl/2/LogicalConstraint")
    }
}

impl LogicConstraint {
    pub fn new(iri: &str) -> Self {
        let uid = String::from(iri);
        LogicConstraint {
            uid: Some(IriBuf::new(uid).unwrap()),
            operator: None,
            operand: None,
        }
    }
    pub fn get_operator(&self) -> Option<ConstraintLogicOperator> {
        self.operator.clone()
    }
    pub fn get_operands(&self) -> Option<Vec<Constraint>> {
        self.operand.clone()
    }
}

impl LogicEval for LogicConstraint {
    fn eval(&self, mut world: &mut StateWorld) -> Result<bool, anyhow::Error> {
        if let None = self.uid {
            return Err(anyhow::Error::msg("No uid defined"));
        }

        if let None = self.operator {
            return Err(anyhow::Error::msg("No operator defined"));
        }

        let operator = self.get_operator().unwrap();
        match operator {
            ConstraintLogicOperator::or => {
                let operands = self.get_operands().unwrap();
                for operand in operands {
                    let ret = operand.eval(&mut world);
                    match &ret {
                        Ok(ret) => {
                            if *ret {
                                return Ok(true);
                            }
                        }
                        Err(e) => {
                        }
                    }
                }
                return Ok(false);
            }
            ConstraintLogicOperator::xone => {
                let operands = &self.get_operands().unwrap();
                let mut count = 0;
                for operand in operands {
                    let ret = operand.eval(&mut world);
                    match &ret {
                        Ok(ret) => {
                            if *ret {
                                count += 1;
                            }
                        }
                        Err(e) => {
                        }
                    }
                }
                return Ok(count == 1);
            }
            ConstraintLogicOperator::andSequence |
            ConstraintLogicOperator::and => {
                let operands = &self.get_operands().unwrap();
                for operand in operands {
                    let ret = operand.eval(&mut world);
                    match &ret {
                        Ok(ret) => {
                            if !ret {
                                return Ok(false);
                            }
                        }
                        Err(_) => {
                        }
                    }
                }
                return Ok(true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::constraint_right_operand::RightOperandType;
    use super::*;
    #[test]
    fn test_constraint() {
        let mut world = StateWorld::default();
        world.add_state("version", "1.0");

        let mut constraint = Constraint::new("http://www.w3.org/ns/odrl/2/Constraint");
        let op: ConstraintOperator = "eq".try_into().unwrap();
        let left: ConstraintLeftOperand = "version".try_into().unwrap();
        let right: ConstraintRightOperand = ConstraintRightOperand::builder()
                                            .value(Some("1.0".to_string()))
                                            .ty(RightOperandType::Literal)
                                            .build();
        constraint.set_operator(Some(op));
        constraint.set_dataType("string".to_string());
        constraint.set_leftOperand(Some(left));
        constraint.set_rightOperand(Some(right));

        let ret =  constraint.eval(&mut world).unwrap();
        assert!(ret);
    }

    #[test]
    fn test_logic_constraint() {
        let mut world = StateWorld::default();
        world.add_state("version", "1.0");

        let mut constraint1 = Constraint::new("http://www.w3.org/ns/odrl/2/Constraint");
        let op: ConstraintOperator = "eq".try_into().unwrap();
        let left: ConstraintLeftOperand = "version".try_into().unwrap();
        let right: ConstraintRightOperand = ConstraintRightOperand::builder()
                                           .value(Some("1.0".to_string()))
                                           .ty(RightOperandType::Literal)
                                           .build();
        constraint1.set_operator(Some(op));
        constraint1.set_dataType("string".to_string());
        constraint1.set_leftOperand(Some(left));
        constraint1.set_rightOperand(Some(right));

        let mut constraint2 = Constraint::new("http://www.w3.org/ns/odrl/2/Constraint");
        let op: ConstraintOperator = "eq".try_into().unwrap();
        let left: ConstraintLeftOperand = "version".try_into().unwrap();
        let right: ConstraintRightOperand = ConstraintRightOperand::builder()
            .value(Some("1.0".to_string()))
            .ty(RightOperandType::Literal)
            .build();
        constraint2.set_operator(Some(op));
        constraint2.set_dataType("string".to_string());
        constraint2.set_leftOperand(Some(left));
        constraint2.set_rightOperand(Some(right));


        let mut logic_constraint = LogicConstraint::builder()
                        .uid(Some(
                            IriBuf::new("http://www.w3.org/ns/odrl/2/LogicalConstraint".to_string()).unwrap(),
                         ))
                        .operator(Some(ConstraintLogicOperator::and))
                        .operand(Some(vec![constraint1, constraint2]))
                        .build();

        let ret =  logic_constraint.eval(&mut world).unwrap();
        assert!(ret);
    }
}


