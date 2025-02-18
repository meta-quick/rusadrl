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

use crate::traits::display::DisplayInfo;

#[derive(Debug,Default,Clone)]
pub struct Metadata {
    pub uri: String,
    pub close_matches: Vec<String>,
    pub labels: Vec<String>,
    pub identifier: String,
    pub comment: String,
    pub see_also: String,
    pub title: String,
    pub note: String,
    pub definition: String,
    pub source: String,
}

impl DisplayInfo for Metadata {
    fn display(&self) {
        println!("Identifier: {}", self.identifier);
        println!("Title: {}", self.title);
        println!("Comment: {}", self.comment);
    }
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            uri: String::new(),
            close_matches: Vec::new(),
            labels: Vec::new(),
            identifier: String::new(),
            comment: String::new(),
            see_also: String::new(),
            title: String::new(),
            note: String::new(),
            definition: String::new(),
            source: String::new(),
        }
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.push(label);
    }

    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    pub fn set_close_matches(&mut self, close_matches: Vec<String>) {
        self.close_matches = close_matches;
    }

    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = identifier;
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }

    pub fn set_see_also(&mut self, see_also: String) {
        self.see_also = see_also;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    pub fn set_definition(&mut self, definition: String) {
        self.definition = definition;
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn get_identifier(&self) -> &String {
        &self.identifier
    }

    pub fn get_comment(&self) -> &String {
        &self.comment
    }

    pub fn get_see_also(&self) -> &String {
        &self.see_also
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_note(&self) -> &String {
        &self.note
    }

    pub fn get_definition(&self) -> &String {
        &self.definition
    }

    pub fn get_source(&self) -> &String {
        &self.source
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }

    pub fn get_close_matches(&self) -> &Vec<String> {
        &self.close_matches
    }
}