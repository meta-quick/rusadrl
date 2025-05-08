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
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use once_cell::sync::Lazy;
use crate::model::asset::AssetCollection;
use crate::model::constraint_right_operand::ConstraintRightOperand;
use crate::model::policy::PolicyUnion;
use crate::traits::definions::WorldCallBack;

#[derive(Default,Builder,Clone)]
pub struct StateWorld {
    pub uid: Option<IriBuf>,
    pub state: HashMap<String, String>,
    pub worldInitialTime: i64,
    pub last_executeTime: i64,
    pub meteredTime: i64,
    pub operand_referred: HashMap<String,ConstraintRightOperand>,
    pub assets: HashMap<String, AssetCollection>,
    pub global_policies: HashMap<String, PolicyUnion>,
    pub success_callback: Option<Arc<Mutex<Vec<Box<dyn WorldCallBack>>>>>,
    pub failure_callback: Option<Arc<Mutex<Vec<Box<dyn WorldCallBack>>>>>,
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
        me.meteredTime = 0;
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

    pub fn update_last_execute_time(&mut self) {
        self.last_executeTime = self.now();
    }

    pub fn timeInterval(&self) -> i64 {
        let last = self.last_executeTime;
        let now = self.now();
        now - last
    }

    pub fn update_metered_time(&mut self, time: i64) {
        self.meteredTime += time;
    }

    pub fn meteredTime(&self) -> i64 {
        self.meteredTime
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

    pub fn add_callback(&mut self, callback: Box<dyn WorldCallBack>, success: bool) {
        if success {
            if let None = self.success_callback {
                self.success_callback = Some(Arc::new(Mutex::new(Vec::new())));
            }
            let callbacks = self.success_callback.clone().unwrap();
            let mut callbacks = callbacks.lock().unwrap();
            callbacks.push(callback);
        } else {
            if let None = self.failure_callback {
                self.failure_callback = Some(Arc::new(Mutex::new(Vec::new())));
            }
            let callbacks = self.failure_callback.clone().unwrap();
            let mut callbacks = callbacks.lock().unwrap();
            callbacks.push(callback);
        }
    }

    pub fn trigger_callback_on_failure(&mut self) {
        let callbacks = self.failure_callback.take();
        if let None = callbacks {
            return;
        }
        let callbacks = callbacks.unwrap();
        let callbacks = callbacks.lock();
        if let Ok(mut callbacks) = callbacks {
            for callback in callbacks.iter_mut() {
                let _ = callback.on_failure(self);
            }
        }
    }

    pub fn trigger_callback_on_success(&mut self) {
        let callbacks = self.success_callback.take();
        if let None = callbacks {
            return;
        }
        let callbacks = callbacks.unwrap();
        let callbacks = callbacks.lock();
        if let Ok(mut callbacks) = callbacks {
            for callback in callbacks.iter_mut() {
                let _ = callback.on_success(self);
            }
        }
    }
}

#[derive(Builder,Clone,Setter,Getter,GetterMut)]
pub struct WorldCache {
    cache: DashMap<String, StateWorld>,
}

impl WorldCache {
    pub fn find_world(&self, iri: &str) -> Option<RefMut<String, StateWorld>> {
        self.cache.get_mut(iri)
    }
    pub fn add_world(&self, iri: &str, world: StateWorld) {
        self.cache.insert(iri.to_string(), world);
    }
    pub fn remove_world(&self, iri: &str) {
        self.cache.remove(iri);
    }
    pub fn update_world(&self, iri: &str, world: StateWorld) {
        self.cache.insert(iri.to_string(), world);
    }

    pub fn clear_world(&self) {
        self.cache.clear();
    }
}

pub static GLOBAL_WORLD_CACHE: Lazy<Arc<WorldCache>> = Lazy::new(|| {
    Arc::new(
        WorldCache {
            cache: DashMap::new(),
    })
});


#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use serde::de;
    use crate::model::stateworld::{StateWorld, GLOBAL_WORLD_CACHE};

    #[test]
    fn test_state_world() {
        #[derive(Debug)]
        struct State {
            uid: String,
            state: String,
        }

        let state = State {
            uid: String::from("foo"),
            state: String::from("bar"),
        };
        let cache = DashMap::<String, State>::new();

        {

            cache.insert(String::from("foo"), state);

            let value = cache.get("foo");
            println!("{:?}", value.unwrap().value());
        }

        //get and modify
        {
            let mut value = cache.get_mut("foo").unwrap();
            let value = value.value_mut();
            value.state = String::from("baz");
            println!("{:?}", value.state);
        }

        {
            let value = cache.get("foo").unwrap();
            println!("{:?}", value.value());
        }
    }

    #[test]
    pub fn test_world_cache() {
        let mut state = StateWorld::default();
        state.worldInitialTime = 11;
        {
            GLOBAL_WORLD_CACHE.cache.insert(String::from("foo"), state);
            let value = GLOBAL_WORLD_CACHE.cache.get("foo");
            println!("{:?}", value.unwrap().worldInitialTime);
        }

        {
            let mut value = GLOBAL_WORLD_CACHE.cache.get_mut("foo").unwrap();
            let value = value.value_mut();
            value.worldInitialTime = 12;
        }

        {
            let value = GLOBAL_WORLD_CACHE.cache.get("foo");
            println!("{:?}", value.unwrap().worldInitialTime);
        }


        {
            let cache = GLOBAL_WORLD_CACHE.clone();
            let value = cache.cache.get("foo");
            println!("{:?}", value.unwrap().worldInitialTime);
        }

        {
            let cache = GLOBAL_WORLD_CACHE.clone();
            let mut value = cache.cache.get_mut("foo").unwrap();
            let value = value.value_mut();
            value.worldInitialTime = 13;
        }

        {
            let value = GLOBAL_WORLD_CACHE.cache.get("foo");
            println!("{:?}", value.unwrap().worldInitialTime);
        }
    }
}
