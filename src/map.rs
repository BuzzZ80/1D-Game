use crate::barrier::*;

pub struct Map {
    pub barriers: Vec<Barrier>,
}

impl Map {
    pub fn empty() -> Self {
        Self { barriers: vec![] }
    }
}
