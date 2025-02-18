#![allow(dead_code)]
use crate::model::metadata::Metadata;
use crate::model::policy::Policy;

#[derive(Debug, Default, Clone)]
pub struct AssetCollection {
    pub metadata: Metadata,
    pub assets: Vec<Asset>,
}

#[derive(Debug,Default, Clone)]
pub struct Asset {
    pub metadata: Metadata,
    pub part_of: Vec<AssetCollection>,
    pub has_policy: Vec<Policy>,
}

impl Asset {
    pub fn new(uri: String) -> Self {
        let mut asset =  Asset {
            metadata: Metadata::new(),
            part_of: Vec::new(),
            has_policy: Vec::new(),
        };
        asset.metadata.set_uri(uri);
        asset
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn get_metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn get_part_of(&self) -> &Vec<AssetCollection> {
        &self.part_of
    }

    pub fn get_part_of_mut(&mut self) -> &mut Vec<AssetCollection> {
        &mut self.part_of
    }

    pub fn get_has_policy(&self) -> &Vec<Policy> {
        &self.has_policy
    }

    pub fn get_has_policy_mut(&mut self) -> &mut Vec<Policy> {
        &mut self.has_policy
    }

    pub fn add_part_of(&mut self, asset_collection: AssetCollection) {
        self.part_of.push(asset_collection);
    }

    pub fn clear_policy(&mut self) {
        self.has_policy.clear();
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.has_policy.push(policy);
    }
}