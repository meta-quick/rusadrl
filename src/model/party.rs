#![allow(dead_code)]

use crate::model::metadata::Metadata;
use crate::model::constraint::Constraint;

#[derive(Debug, Default, Clone)]
pub struct PartyCollection {
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone)]
pub struct Party {
    pub metadata: Metadata,
    pub part_of: Vec<PartyCollection>,
    pub refinements: Vec<Constraint>,
}

impl Party {
    pub fn new() -> Self {
        Party {
            metadata: Metadata::new(),
            part_of: Vec::new(),
            refinements: Vec::new(),
        }
    }
}