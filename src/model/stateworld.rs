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


use std::collections::{HashMap, VecDeque};
use std::sync::{Arc};
use chrono::Duration;
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};
use once_cell::sync::Lazy;
use crate::model::asset::{AssetCollection};
use crate::model::constraint_left_operand::parse_xml_duration;
use crate::model::constraint_right_operand::ConstraintRightOperand;
use crate::model::policy::{PolicyUnion};

#[derive(Debug,Default,Builder,Clone)]
pub struct StateWorld {
    pub uid: Option<IriBuf>,
    pub state: HashMap<String, String>,
    pub worldInitialTime: i64,
    pub last_executeTime: i64,
    pub meteredTime: i64,
    pub counter_sequence: VecDeque<i64>,
    pub slide_window_duration: i64,
    pub slide_window_counter: i64,
    pub enabled_slide_window: bool,
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
        me.meteredTime = 0;
        me.enabled_slide_window = false;
        me
    }
    pub fn add_state(&mut self, state: &str, value: &str) {
        //Hack for timeWindow
        if state.contains("timeWindow") {
            self.set_slide_window(value.to_string());
            self.enabled_slide_window = true;
            return;
        }

        self.state.insert(state.to_string(), value.to_string());
    }

    pub fn remove_state(&mut self, state: &str) {
        if state.contains("timeWindow") {
            self.enabled_slide_window = false;
            return;
        }

        self.state.remove(state);
    }
    pub fn get_state(&self, state: &str) -> Option<&str> {
        self.state.get(state).map(|s| s.as_ref())
    }
    pub fn update_state(&mut self, state: &str, value: &str) {
        //Hack for timeWindow
        if state.contains("timeWindow") {
            self.set_slide_window(value.to_string());
            self.enabled_slide_window = true;
            return;
        }

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

    pub fn set_slide_window(&mut self, slide: String) {
        //slide format: count/duration
        let parts = slide.split('/').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return;
        }

        let count = parts[0].parse::<i64>();
        if count.is_err() {
            return;
        }
        let count = count.unwrap();

        let duration = parse_xml_duration(parts[1]);
        if duration.is_err() {
            return;
        }
        let duration = duration.unwrap();

        self.set_slide_window_duration(count, duration);
    }

    pub fn set_slide_window_duration(&mut self, count : i64, duration: Duration) {
        self.slide_window_duration = duration.num_milliseconds();
        self.slide_window_counter = count;
    }

    pub fn update_slide_window(&mut self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.counter_sequence.push_back(now);
    }

    pub fn calc_slide_window(&mut self) -> i64 {
        //calc left window
        let now = chrono::Utc::now().timestamp_millis();
        //check if the window is expired
        loop{
            let first = self.counter_sequence.get(0);
            if first.is_none() {
                break;
            }
            if now - first.unwrap() > self.slide_window_duration {
                self.counter_sequence.pop_front();
            } else {
                break;
            }
        }

        self.slide_window_counter - self.counter_sequence.len() as i64
    }
}

#[derive(Debug,Builder,Clone,Setter,Getter,GetterMut)]
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
    use super::*;
    #[test]
    fn test_state_world() {
        let mut w = StateWorld::new("http://example.com/");
        w.set_slide_window_duration(10, Duration::milliseconds(1000));

        for i in 0..6 {
            std::thread::sleep(core::time::Duration::from_millis(10));
            w.update_slide_window();
        }

        // std::thread::sleep(core::time::Duration::from_millis(960));

        let count = w.calc_slide_window();
        assert_eq!(count, 4);
    }
}
