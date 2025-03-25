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
    pub status: Option<String>,

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
            status: None,
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
        let right = self.rightOperand.as_ref();
        if right.is_none() {
            return Err(anyhow::Error::msg("No right operand defined"));
        }
        let right = right.unwrap();

        let left_value = left.value(&mut world);
        if left_value.is_err() {
            if self.status.is_none() {
               return   Ok(false);
            }
        }

        let left_value = left_value.unwrap();

        let right_value = right.value(&mut world);
        if right_value.is_err() {
           return   Ok(false);
        }
        let right_value = right_value.unwrap();

        let dty = DataType::try_from(self.dataType.clone());
        match dty {
            Ok(dty) => {
                let result = operator.eval(dty,&left_value, &right_value, &self.status);
                Ok(result)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}


#[derive(Debug,Builder,Getter,GetterMut,Setter,Default, Clone)]
pub struct LogicConstraint {
    pub uid: Option<IriBuf>,
    pub operator: Option<ConstraintLogicOperator>,
    pub operand: Option<Vec<Constraint>>
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

        let operator = self.get_operator().clone().unwrap();
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
                        Err(_) => {
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
                        Err(_) => {
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

#[derive(Debug, Clone)]
pub enum ConstraintUnion {
    Constraint(Constraint),
    LogicConstraint(LogicConstraint),
}


pub struct ConstraintInference;

impl ConstraintInference {
    pub fn infer(world: &StateWorld, constraints: &Vec<ConstraintUnion>) -> Result<bool,anyhow::Error> {
        let mut result = true;
        for constraint in constraints {
            let ret =  ConstraintInference::infer_one(world, constraint);
            match ret {
                Ok(false) => { result = false; },
                _ => {
                }
            }
        }

        Ok(result)
    }
    pub fn infer_one(world: &StateWorld, constraint: &ConstraintUnion) -> Result<bool,anyhow::Error> {
        let mut result = true;
        match constraint {
            ConstraintUnion::Constraint(c) => {
                let mut world = world.clone();
                let ret = c.eval(&mut world);
                match ret {
                    Ok(false) => { result = false; },
                    _ => {
                    }
                }
            }
            ConstraintUnion::LogicConstraint(lc) => {
                let mut world = world.clone();
                let ret = lc.eval(&mut world);
                match ret {
                    Ok(false) => { result = false; },
                    _ => {
                    }
                }
            }
        }

        Ok(result)
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


        let logic_constraint = LogicConstraint::builder()
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


