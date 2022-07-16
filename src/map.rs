use crate::light::Light;
use crate::barrier::*;

pub struct Map {
    pub barriers: Vec<Barrier>,
    pub lights: Vec<Light>,
}

impl Map {
    pub fn empty() -> Self {
        Self { 
            barriers: vec![],
            lights: vec![],
        }
    }
}
