mod block;
mod map;
mod lines;
mod camera;
mod frame_buffer;

use lines::*;
use camera::*;
use frame_buffer::FrameBuffer;
use map::Map;
use minifb::Key; // For getting keyboard input

const PIXELS_PER_SCAN: usize = 1;

fn main() {
    // Create new window and buffer
    let mut fb = FrameBuffer::new("1D Game");
    let mut _map = Map::empty();

    fb.window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let ray = Ray {
        pos: Point(0.0, 0.5),
        dir: Point(1.0, 0.0),
    };

    match ray.cast_for_block_at((1.0, 0.0)) {
        Some(p) => {
            println!("INTERSECTION at ({}, {})", p.0, p.1);
            for x in 0..256 {
                fb.fill_column(x, (x * 0x010101) as u32);
            }
        }
        None => println!("No intersection :("),
        
    }

    while fb.window.is_open() && !fb.window.is_key_down(Key::Escape) {
        fb.update();
    }
}
