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

use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::model::stateworld::StateWorld;
use crate::reference::types::{OperandValue, OperandValueType};

#[derive(Debug,Default, Clone)]
pub enum RightOperandType {
    #[default]
    Literal,
    LiteralSet,
    Reference
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct RightOperandReference {
    pub reference: Option<IriBuf>
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct ConstraintRightOperand {
    pub ty: RightOperandType,
    pub value: Option<String>,
    pub values: Option<Vec<String>>,
    pub reference: Option<RightOperandReference>
}

impl  ConstraintRightOperand {
   pub fn value(&self, world: &mut StateWorld) -> Result<OperandValue,anyhow::Error> {
        match &self.ty {
            RightOperandType::Literal => {
                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::string);
                val.set_sval(self.value.clone());
                Ok(val)
            },
            RightOperandType::LiteralSet => {
                let mut val = OperandValue::default();
                val.set_ty(OperandValueType::set);

                let mut set:Vec<String> = vec![];
                if let Some(values) = &self.values {
                    for value in values {
                        set.push(value.clone());
                    }
                }
                val.set_sets(Some(set));
                Ok(val)
            },
            RightOperandType::Reference => {
                if let Some(reference) = &self.reference {
                    if let Some(reference) = &reference.reference {
                        let iri = reference.as_str();
                        let referred = world.get_referred_operand(iri);

                        if let Some(referred) = referred {
                            match referred.ty {
                                RightOperandType::Literal => {
                                    let mut val = OperandValue::default();
                                    val.set_ty(OperandValueType::string);
                                    val.set_sval(self.value.clone());
                                    return Ok(val);
                                },
                                RightOperandType::LiteralSet => {
                                    let mut val = OperandValue::default();
                                    val.set_ty(OperandValueType::set);

                                    let mut set:Vec<String> = vec![];
                                    if let Some(values) = &referred.values {
                                        for value in values {
                                            set.push(value.clone());
                                        }
                                    }
                                    val.set_sets(Some(set));
                                    return Ok(val);
                                }
                                _ => {
                                    return Err(anyhow::anyhow!("Unsupported operand type"));
                                }
                            }
                        }
                    }
                }
                Err(anyhow::anyhow!("Unsupported operand type"))
            }
        }
    }
}