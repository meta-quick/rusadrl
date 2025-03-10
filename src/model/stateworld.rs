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


use std::collections::HashMap;
use iref::IriBuf;
use lombok::{Builder, Getter, GetterMut, Setter};

#[derive(Debug,Default,Builder,Getter,GetterMut,Setter, Clone)]
pub struct StateWorld {
    pub uid: Option<IriBuf>,
    pub state: HashMap<String, String>,
    pub worldInitialTime: i64,
    pub last_executeTime: i64,
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

    pub fn eclispsed_datatime(&self) -> i64 {
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
}