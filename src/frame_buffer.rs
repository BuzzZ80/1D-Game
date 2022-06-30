use minifb::{ScaleMode, Window, WindowOptions}; // For opening the window and its framebuffer

const WIDTH: usize = 512;
const HEIGHT: usize = 300;

pub struct FrameBuffer {
    pub window: Window,
    pub buffer: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(title: &str) -> Self {
        Self {
            window: Window::new(
                title,
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: false,
                    scale_mode: ScaleMode::UpperLeft,
                    ..WindowOptions::default()
                },
            )
            .expect("Unable to create window"),
            buffer: vec![0u32; WIDTH * HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let w = self.window.get_size().0;
        self.buffer[x + y * w] = color;
    }

    pub fn fill_column(&mut self, x: usize, color: u32) {
        for y in 0..self.window.get_size().1 {
            self.set_pixel(x, y, color)
        }
    }

    pub fn update(&mut self) {
        let (w, h) = self.window.get_size();
        self.window.update_with_buffer(&self.buffer, w, h).unwrap();
    }
}
