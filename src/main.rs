mod frame_buffer;
mod lines;
mod barrier;
mod map;
mod camera;

use minifb::Key; // For getting keyboard input
use frame_buffer::FrameBuffer;
use lines::*;
use barrier::*;
use map::Map;
use camera::*;

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
        color: 0xFFFFFF,
        kind: BarrierKind::Basic,
        seg: Segment(Point(1.0, 0.0), Point(2.0, 1.0)),
    });

    cam.capture(&map);

    for (i, c) in cam.view.iter().enumerate() {
        fb.fill_column(i, *c)
    }

    /*let ray = Ray {
        pos: Point(0.0, 0.5),
        dir: Direction(1.0, 0.0),
    };

    match ray.cast_for_segment(&Segment(Point(1.0, 0.0), Point(1.0, 1.0))) {
        Some(p) => {
            println!("INTERSECTION at ({}, {})", p.point.0, p.point.1);
            println!("INTERSECTION at {} block/s away", p.distance);
            println!("INTERSECTION at {} radians", p.angle.0);
            for x in 0..256 {
                fb.fill_column(x, (x * 0x010101) as u32);
            }
        }
        None => println!("No intersection :("),
    }*/

    while fb.window.is_open() && !fb.window.is_key_down(Key::Escape) {
        fb.update();
    }
}
