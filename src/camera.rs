use crate::lines::*;
use crate::map::Map;

pub struct Camera {
    pub pos: Point,
    pub ang: Angle,
    pub fov: Angle,
    pub view: Vec<u32>,
}

impl Camera {
    pub fn new(pos: Point, ang: Angle, fov: Angle, rays: usize) -> Self {
        Self {
            pos,
            ang,
            fov,
            view: vec![0; rays],
        }
    }

    pub fn capture(&mut self, env: &Map) {
        let start_angle = self.ang.0 - (self.fov.0 / 2.0);
        let step_angle = self.fov.0 / self.view.len() as f32;

        for (i, c) in self.view.iter_mut().enumerate() {
            let current_angle = start_angle + step_angle * i as f32;
            let ray = Ray {
                pos: self.pos,
                dir: Direction(current_angle.cos(), current_angle.sin()),
            };

            *c = match ray.cast_for_map(env) {
                Some(intersection) => {
                    let mut color = intersection.barrier.unwrap().color;
                    color = color * f32::min(5.0 / intersection.distance, 1.0);
                    color = color * (intersection.angle.0.sin());

                    color.into()
                }
                None => 0
            }

        }
    }
}
