use crate::block::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 15;

pub struct Map<'a> {
    pub blocks: [[&'a Block; WIDTH]; HEIGHT],
}

impl<'a> Map<'a> {
    pub fn empty() -> Self {
        Self {
            blocks: [[&AIR; WIDTH]; HEIGHT],
        }
    }
}
