use super::color::Color;
use crate::vector::Vec2D;

// --------------------------------------------------

#[derive(Clone, Debug)]
pub struct FrameBuffer {
    buffer: Vec<Color>,
    size: Vec2D<u32>,
}

#[allow(dead_code)]
impl FrameBuffer {
    pub fn new(size: Vec2D<u32>) -> FrameBuffer {
        FrameBuffer {
            buffer: vec![Color::new(0, 0, 0); (size.x * size.y) as usize],
            size,
        }
    }

    pub const fn new_dummy(size: Vec2D<u32>) -> FrameBuffer {
        FrameBuffer {
            buffer: vec![],
            size,
        }
    }

    pub const fn size(&self) -> &Vec2D<u32> {
        &self.size
    }

    pub const fn buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    pub fn set_buffer(&mut self, buffer: &Vec<Color>) -> Result<(), &str> {
        if self.buffer.len() == buffer.len() {
            self.buffer = (*buffer).clone();
            Ok(())
        } else {
            Err("Incorrect length of buffer")
        }
    }

    pub fn contains_point(&self, p: Vec2D<f32>) -> bool {
        p.x >= 0.0 && p.x < (self.size.x as f32) && p.y >= 0.0 && p.y < (self.size.y as f32)
    }

    pub fn set_pixel(&mut self, p: Vec2D<f32>, color: Color) {
        if self.contains_point(p) {
            self.buffer[((p.y * (self.size.x as f32)) + p.x) as usize] = color;
        }
    }

    pub fn to_vec_u8(&self, transparency: bool) -> (Vec<u8>, Vec2D<u32>) {
        let mut out: Vec<u8> =
            Vec::with_capacity(self.buffer.len() * (if transparency { 4 } else { 3 }));

        for i in 0..self.buffer.len() {
            let current_color = self.buffer[i];

            out.push(current_color.r());
            out.push(current_color.g());
            out.push(current_color.b());
            if transparency {
                out.push(255);
            }
        }
        (out, self.size)
    }

    pub fn to_vec_u32(&self) -> Vec<u32> {
        self.buffer.iter().map(|x| x.to_u32()).collect()
    }
}
