#![allow(dead_code)]

pub struct AnnoId {
    pub id: String,
}

impl AnnoId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}