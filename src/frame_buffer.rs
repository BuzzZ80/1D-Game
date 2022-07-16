use minifb::{ScaleMode, Window, WindowOptions}; // For opening the window and its framebuffer

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 300;

pub struct FrameBuffer {
    pub window: Window,
    pub buffer: Vec<u32>,
}

#[derive(Debug, Copy, Clone)]
pub struct Color(pub f32, pub f32, pub f32);

impl FrameBuffer {
    pub fn new(title: &str) -> Self {
        Self {
            window: Window::new(
                title,
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: true,
                    scale_mode: ScaleMode::UpperLeft,
                    ..WindowOptions::default()
                },
            )
            .expect("Unable to create window"),
            buffer: vec![0u32; WIDTH * HEIGHT],
        }
    }

    pub fn fill_column(&mut self, x: usize, color: u32) {
        let column_offset = self.window.get_size().0;
        let mut y = 0;
        for _ in 0..self.window.get_size().1 {
            self.buffer[x + y] = color;
            y += column_offset
        }
    }

    pub fn update(&mut self) {
        let (w, h) = self.window.get_size();
        self.window.update_with_buffer(&self.buffer, w, h).unwrap();
    }
}

impl std::convert::From<Color> for u32 {
    fn from(color: Color) -> u32 {
        (((color.0 * 255.0) as u32) << 16)
            + (((color.1 * 255.0) as u32) << 8)
            + ((color.2 * 255.0) as u32)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        if other == 0.0 {
            Color(0.0, 0.0, 0.0)
        } else {
            Color(self.0 / other, self.1 / other, self.2 / other)
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}
