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
#![warn(non_snake_case)]
use std::collections::HashMap;
use lombok::{Builder, Getter, GetterMut, Setter};
use crate::traits::display::PrettyPrint;

#[derive(Debug,Builder,Setter,Getter,GetterMut,Default,Clone)]
pub struct Metadata {
    pub close_matches: Vec<String>,
    pub tags: HashMap<String, String>,
    pub identifier: String,
    pub comment: String,
    pub see_also: String,
    pub title: String,
    pub note: String,
    pub definition: String,
    pub source: String,
}

impl PrettyPrint for Metadata {
    fn display(&self) -> String {
        let mut sb = String::new();
        sb.push_str("---------metadata---------\n");
        sb.push_str(&format!("  Identifier: {}\n", self.identifier));
        sb.push_str(&format!("  Title: {}\n", self.title));
        sb.push_str(&format!("  Comment: {}\n", self.comment));
        sb.push_str(&format!("  See Also: {}\n", self.see_also));
        sb.push_str(&format!("  Definition: {}\n", self.definition));
        sb.push_str(&format!("  Source: {}\n", self.source));
        sb.push_str(&format!("  Tags: {}\n", "{"));

        for (k, v) in self.tags.iter() {
            sb.push_str(&format!("   {}: {}\n", k, v));
        }
        sb.push_str(&format!("  {}", "}"));

        sb
    }
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            close_matches: Vec::new(),
            tags: HashMap::new(),
            identifier: String::new(),
            comment: String::new(),
            see_also: String::new(),
            title: String::new(),
            note: String::new(),
            definition: String::new(),
            source: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let mut m = Metadata::new();
        m.set_identifier("test".to_string());
        m.set_title("test".to_string());
        m.set_comment("test".to_string());
        m.set_see_also("test".to_string());
        m.set_definition("test".to_string());
        m.set_source("test".to_string());
        let mut tags: HashMap<String, String> = HashMap::new();
        tags.insert("test".to_string(), "test".to_string());
        tags.insert("name".to_owned(), "test".to_owned());
        m.set_tags(tags);
        println!("{}", m.display());
    }
}