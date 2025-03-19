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
use iref::IriBuf;
use json_ld::Compact;
use json_ld::context_processing::ProcessedRef;
use json_ld::object::Any;
use json_ld_core::{ExpandedDocument};
use json_ld_core::Node;
use serde_json::Value;
use static_iref::iri;
use crate::linkdata::jsonld::{JsonLdParser, JsonLdParty, JsonLdPolicy};
use crate::model::constraint::Constraint;
use crate::model::constraint::LogicConstraint;
use crate::model::{constraint::ConstraintUnion};
use crate::model::constraint_left_operand::ConstraintLeftOperand;
use crate::model::constraint_operator::ConstraintOperator;
use crate::model::constraint_right_operand::{ConstraintRightOperand, RightOperandType};

#[derive(Debug)]
pub struct OdrlLoader;

impl OdrlLoader {
    pub async  fn load(iri: String, path: String) -> Result<ExpandedDocument, anyhow::Error> {
        let mut parse = JsonLdParser::new(None);
        let val = std::fs::read_to_string(path)?;

        let document = parse.parse(iri, val.to_string()).await;
        document
    }

    pub async fn parse(expanded: ExpandedDocument) -> Result<JsonLdPolicy, anyhow::Error> {
        let unprocessed = json_ld_syntax::context::Context::null();
        let processed = json_ld_core::context::Context::new(
            Some(IriBuf::new("http://example.org/".to_owned()).unwrap())
        );
        let context = ProcessedRef::new(&unprocessed, &processed);
        let result = expanded.compact(context, &mut json_ld::NoLoader::default()).await;

        match result {
            Ok(compacted) => {
                let json_string = compacted.to_string();
                let v: Value = serde_json::from_str(&json_string).unwrap();
                let pretty = serde_json::to_string_pretty(&v).unwrap();
                println!("{}", pretty);

                let policy =   serde_json::from_str::<JsonLdPolicy>(&json_string);
                let policy = match policy {
                    Ok(policy) => policy,
                    Err(err) => {
                        return Err(anyhow!("Error during compaction: {err:#?}"));
                    }
                };

                // println!("{:#?}",policy);
                Ok(policy)
            }
            Err(err) => {
                Err(anyhow!("Error during compaction: {err:#?}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn test() {
        let loader = OdrlLoader {};
        let path = "src/data/sample.jsonld";
        let doc = OdrlLoader::load("http://www.w3.org/ns/odrl/2".to_string(), path.to_string());
        let doc = doc.await;
        let expanded = doc.unwrap();

        let policy = OdrlLoader::parse(expanded).await;
        println!("{:#?}", policy);
    }
}