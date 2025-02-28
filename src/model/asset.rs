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

use crate::model::metadata::Metadata;
use crate::traits::validate::Validate;

use super::constraint::Constraint;
use super::error::OdrlError;

#[derive(Debug,Builder,Getter,GetterMut,Setter, Default, Clone)]
pub struct AssetCollection {
    pub source  : Option<IriBuf>,
    pub refinement: Option<Vec<Constraint>>,
    pub metadata: Metadata,
}

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct Asset {
    pub uid: Option<IriBuf>,
    //part of the asset collection
    pub partOf: Option<Vec<IriBuf>>,
    //refer to policy definition by IRI of Policy
    pub hasPolicies: Option<IriBuf>,
    pub metadata: Option<Metadata>,
}

impl Validate for Asset {
    fn validate(&self) -> Result<(), OdrlError> {
        let uid = self.get_uid();
        match uid {
            Some(_) => {
                Ok(())
            },
            None => {
                Err(OdrlError::InvalidAssetIRI)
            }
        }
    }
}

impl Asset {
    pub fn new(uri: String) -> Self {
        let mut asset = Asset::default();
        asset.set_uid(Some(IriBuf::new(uri).unwrap()));
        asset
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asset_validation() {
        let asset = Asset::default();
        assert!(asset.validate().is_err());
    }

    #[test]
    fn test_valid_asset() {
        let uri = "http://data.org/1".to_string();
        let asset = Asset::new(uri);
        assert!(asset.validate().is_ok());
    }
}