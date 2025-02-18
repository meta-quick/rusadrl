#![allow(dead_code)]

use crate::traits::definions::{Model, Resource};

pub struct ResourceModel {
    pub resource: Box<dyn Resource>,
    pub model: Box<dyn Model>,
}