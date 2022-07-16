use crate::frame_buffer::Color;
use crate::lines::*;

pub struct Light {
    pub pos: Point,
    pub intensity: f32,
    pub color: Color,
}