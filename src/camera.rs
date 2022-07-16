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
        let len = self.view.len() as f32;

        for (i, c) in self.view.iter_mut().enumerate() {
            let current_angle = start_angle + step_angle * i as f32;
            let ray = Ray {
                pos: self.pos,
                dir: Direction(current_angle.cos(), current_angle.sin()),
            };

            *c = match ray.cast_for_map(env) {
                Some(intersection) => {
                    let mut color = crate::Color(0.0, 0.0, 0.0);
                    // color = color * (intersection.angle.0.sin());
                    for light in &env.lights {
                        if light.pos.can_see(&env, intersection.pos) {
                            let ray = Ray::new_towards_point(intersection.pos, light.pos);
                            let light_reflect_angle = Angle::from(ray.dir).0 - intersection.barrier.unwrap().seg.get_angle().0; // Get the angle the light bounces off (angle of light relative to angle of surface)
                            
                            color = color + intersection.barrier.unwrap().color // Color of barrier
                                * light.color // Color of light
                                * f32::min(1.0, light.intensity / (light.pos.distance_from(intersection.pos)).powi(2)) // Inverse square law (but spread out more)
                                * f32::abs(light_reflect_angle.sin());    // Surface should look darker if angle of the reflection of light is extreme
                        }
                    }
                    color = color * intersection.angle.0.sin();
                    color = color * f32::min(1.0, 10.0 / intersection.dist.powi(2));

                    color.into()
                }
                None => 0,
            }
        }
    }
}
