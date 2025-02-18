#![allow(dead_code)]
#![allow(dead_code)]

use crate::model::constraint::Constraint;
use crate::model::metadata::Metadata;
use crate::traits::display::DisplayInfo;

#[derive(Debug, Clone)]
pub struct Action {
    pub metadata: Metadata,
    pub refinements: Vec<Constraint>,
}

impl DisplayInfo for Action {
    fn display(&self) {
        self.metadata.display();
        println!("Number of refinements: {}", self.refinements.len());
    }
}


impl Action {
    pub fn new() -> Self {
        Action {
            metadata: Metadata::new(),
            refinements: Vec::new(),
        }
    }

    pub fn add_refinement(&mut self, constraint: Constraint) {
        self.refinements.push(constraint);
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn get_refinements(&self) -> &Vec<Constraint> {
        &self.refinements
    }

    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    pub fn set_refinements(&mut self, refinements: Vec<Constraint>) {
        self.refinements = refinements;
    }

    pub fn load_odrl( _uri: &str) {
        // let actions = Vec::<Action>::new();
    }
}