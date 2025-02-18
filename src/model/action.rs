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