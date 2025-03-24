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

mod config;
pub mod model;
mod traits;
mod reference;
mod linkdata;

pub use config::*;
pub use linkdata::odrl_loader;

pub fn handle_to_policy<'a>(handle: *mut i64) -> Option<&'a mut crate::model::policy::PolicyUnion> {
    if handle.is_null() {
        return None;
    }
    let odrl = unsafe {
        &mut *(handle as *mut crate::model::policy::PolicyUnion)
    };
    return Some(odrl);
}