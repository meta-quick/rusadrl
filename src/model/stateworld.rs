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


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use once_cell::sync::Lazy;
use crate::model::asset::{AssetCollection};
use crate::model::constraint_right_operand::ConstraintRightOperand;
use crate::model::policy::{PolicyUnion};

#[derive(Debug,Default,Builder,Clone)]
pub struct StateWorld {
    pub uid: Option<IriBuf>,
    pub state: HashMap<String, String>,
    pub worldInitialTime: i64,
    pub last_executeTime: i64,
    pub operand_referred: HashMap<String,ConstraintRightOperand>,
    pub assets: HashMap<String, AssetCollection>,
    pub global_policies: HashMap<String, PolicyUnion>,
}

impl StateWorld {
    pub fn new(iri: &str) -> Self {
        let mut me = Self::default();
        let iri = IriBuf::new(iri.to_owned());
        match iri {
            Ok(iri) => {
                me.uid = Some(iri);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        me.worldInitialTime = chrono::Utc::now().timestamp_millis();
        me
    }
    pub fn add_state(&mut self, state: &str, value: &str) {
        self.state.insert(state.to_string(), value.to_string());
    }

    pub fn remove_state(&mut self, state: &str) {
        self.state.remove(state);
    }
    pub fn get_state(&self, state: &str) -> Option<&str> {
        self.state.get(state).map(|s| s.as_ref())
    }
    pub fn update_state(&mut self, state: &str, value: &str) {
        self.state.insert(state.to_string(), value.to_string());
    }

    pub fn eclipse_datetime(&self) -> i64 {
        let now = chrono::Utc::now().timestamp_millis();
        now - self.worldInitialTime
    }

    pub fn now(&self) -> i64 {
        let now = chrono::Utc::now().timestamp_millis();
        now
    }

    pub fn last_execute_time(&self) -> i64 {
        self.last_executeTime
    }

    pub fn set_referred_operand(&mut self, iri: String, referred: ConstraintRightOperand) {
        self.operand_referred.insert(iri, referred);
    }

    pub fn get_referred_operand(&self, iri: &str) -> Option<ConstraintRightOperand> {
        self.operand_referred.get(iri).cloned()
    }
    pub fn get_assets(&self,iri: String) -> Option<AssetCollection> {
        self.assets.get(&iri).cloned()
    }

    pub fn add_assets(&mut self,iri: String,assets: AssetCollection) {
        self.assets.insert(iri.to_string(), assets);
    }

    pub fn get_policy(&self,iri: String) -> Option<PolicyUnion> {
        self.global_policies.get(&iri).cloned()
    }

    pub fn add_policy(&mut self,iri: String,policy: PolicyUnion) {
        self.global_policies.insert(iri.to_string(), policy);
    }
}

#[derive(Debug,Builder,Clone,Setter,Getter,GetterMut)]
pub struct WorldCache {
    cache: HashMap<String, StateWorld>,
}

impl WorldCache {
    pub fn find_world(&self, iri: &str) -> Option<&StateWorld> {
        self.cache.get(iri)
    }
    pub fn add_world(&mut self, iri: &str, world: StateWorld) {
        self.cache.insert(iri.to_string(), world);
    }
    pub fn remove_world(&mut self, iri: &str) {
        self.cache.remove(iri);
    }
    pub fn update_world(&mut self, iri: &str, world: StateWorld) {
        self.cache.insert(iri.to_string(), world);
    }

    pub fn clear_world(&mut self) {
        self.cache.clear();
    }
}

pub static GLOBAL_WORLD_CACHE: Lazy<Arc<Mutex<WorldCache>>> = Lazy::new(|| {
    Arc::new(Mutex::new( {
        WorldCache {
            cache: HashMap::new(),
        }
    }))
});



