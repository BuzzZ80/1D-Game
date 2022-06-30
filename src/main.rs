mod frame_buffer;
mod lines;
mod barrier;
mod map;
mod camera;

use minifb::Key; // For getting keyboard input
use frame_buffer::*;
use lines::*;
use barrier::*;
use map::Map;
use camera::*;

const MOVEMENT_SPEED: f32 = 1.0 / 60.0;
const ROTATION_SPEED: f32 = std::f32::consts::PI / 120.0;

fn main() {
    // Create new window and buffer
    let mut fb = FrameBuffer::new("1D Game");
    let mut _map = Map::empty();

    fb.window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut cam = Camera::new(
        Point(0.0, 0.5),
        Angle(0.0),
        Angle(std::f32::consts::PI / 2.0),
        fb.window.get_size().0,
    );

    let mut map = Map::empty();
    map.barriers.push(Barrier{
        color: Color(1.0, 1.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(1.0, 0.0), Point(2.0, 1.0)),
    });
    map.barriers.push(Barrier{
        color: Color(1.0, 0.0, 0.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(2.0, 1.0), Point(2.0, 2.0)),
    });
    map.barriers.push(Barrier{
        color: Color(1.0, 0.0, 1.0),
        kind: BarrierKind::Basic,
        seg: Segment(Point(3.0, 0.0), Point(3.0, 3.0)),
    });

    while fb.window.is_open() && !fb.window.is_key_down(Key::Escape) {
        fb.window.get_keys().iter().for_each(|key| {
            //let camcos = cam.ang.0.cos();
            //let camsin = cam.ang.0.sin();
            let forwardx = cam.ang.0.cos();
            let forwardy = cam.ang.0.sin();
            let rightx = (cam.ang.0 + std::f32::consts::PI / 2.0).cos();
            let righty = (cam.ang.0 + std::f32::consts::PI / 2.0).sin();
            match key {
                Key::W => cam.pos = cam.pos + Point(forwardx * MOVEMENT_SPEED, forwardy * MOVEMENT_SPEED),
                Key::A => cam.pos = cam.pos + Point(-rightx * MOVEMENT_SPEED, -righty * MOVEMENT_SPEED),
                Key::S => cam.pos = cam.pos + Point(-forwardx * MOVEMENT_SPEED, -forwardy * MOVEMENT_SPEED),
                Key::D => cam.pos = cam.pos + Point(rightx * MOVEMENT_SPEED, righty * MOVEMENT_SPEED),
                Key::Left => cam.ang = Angle(cam.ang.0 - ROTATION_SPEED),
                Key::Right => cam.ang = Angle(cam.ang.0 + ROTATION_SPEED),
                _ => (),
            }}
        );

        println!("CAM_POS: ({}, {}) @ {} radians", cam.pos.0, cam.pos.1, cam.ang.0);

        cam.capture(&map);

        for (i, c) in cam.view.iter().enumerate() {
            fb.fill_column(i, *c)
        }

        fb.update();
    }
}
