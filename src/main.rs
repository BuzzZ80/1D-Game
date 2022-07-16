mod barrier;
mod camera;
mod light;
mod frame_buffer;
mod lines;
mod map;

use barrier::*;
use camera::*;
use light::Light;
use frame_buffer::{Color, FrameBuffer};
use lines::*;
use map::Map;
use minifb::Key; // For getting keyboard input

const MOVEMENT_SPEED: f32 = 1.0 / 60.0;
const ROTATION_SPEED: f32 = std::f32::consts::PI / 180.0;
const FOV_STEP: f32 = 1.01; // std::f32::consts::PI / 120.0;

fn main() {
    // Create new window and buffer
    let mut fb = FrameBuffer::new("1D Game");

    fb.window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cam = Camera::new(
        Point(5.0, 0.5),
        Angle(0.0),
        Angle(std::f32::consts::PI / 2.0),
        fb.window.get_size().0,
    );

    let mut map = Map::empty();
    map.barriers.push(Barrier {
        color: Color(1.0, 1.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(0.0, 0.0), Point(10.0, 0.0)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 1.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(0.0, 0.0), Point(0.0, 10.0)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 1.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(10.0, 0.0), Point(10.0, 10.0)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 1.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(0.0, 10.0), Point(10.0, 10.0)),
    });

    map.barriers.push(Barrier {
        color: Color(1.0, 0.0, 0.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(4.5, 4.5), Point(5.5, 4.5)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 0.0, 0.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(4.5, 4.5), Point(4.5, 5.5)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 0.0, 0.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(5.5, 4.5), Point(5.5, 5.5)),
    });
    map.barriers.push(Barrier {
        color: Color(1.0, 0.0, 0.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(4.5, 5.5), Point(5.5, 5.5)),
    });

    map.lights.push(Light{
        pos: Point(5.0, 1.0),
        intensity: 100.0,
        color: Color(1.0, 1.0, 1.0),
    });
    /*map.lights.push(Light{
        pos: Point(6.0, 5.0),
        color: Color(1.0, 1.0, 1.0),
    });*/

    let mut size = (0, 0);

    while fb.window.is_open() && !fb.window.is_key_down(Key::Escape) {
        let new_size = (fb.window.get_size().0, fb.window.get_size().1);
        if new_size != size {
            size = new_size;
            fb.buffer.resize(size.0 * size.1, 0);
            cam.view.resize(size.0, 0);
        }
        fb.window.get_keys().iter().for_each(|key| {
            //let camcos = cam.ang.0.cos();
            //let camsin = cam.ang.0.sin();
            let forwardx = cam.ang.0.cos() * MOVEMENT_SPEED;
            let forwardy = cam.ang.0.sin() * MOVEMENT_SPEED;
            let rightx = (cam.ang.0 + std::f32::consts::PI / 2.0).cos() * MOVEMENT_SPEED;
            let righty = (cam.ang.0 + std::f32::consts::PI / 2.0).sin() * MOVEMENT_SPEED;
            match key {
                Key::W => {
                    cam.pos = cam.pos + Point(forwardx, forwardy)
                }
                Key::A => {
                    cam.pos = cam.pos + Point(-rightx, -righty)
                }
                Key::S => {
                    cam.pos = cam.pos + Point(-forwardx, -forwardy)
                }
                Key::D => {
                    cam.pos = cam.pos + Point(rightx, righty)
                }
                Key::Left => cam.ang = Angle(cam.ang.0 - ROTATION_SPEED),
                Key::Right => cam.ang = Angle(cam.ang.0 + ROTATION_SPEED),
                Key::Q => cam.fov.0 *= FOV_STEP,
                Key::E => cam.fov.0 /= FOV_STEP,
                _ => (),
            }
        });

        cam.capture(&map);

        for (i, c) in cam.view.iter().enumerate() {
            fb.fill_column(i, *c)
        }

        fb.update();
    }
}
