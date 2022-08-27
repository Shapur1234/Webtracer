use crate::vector::Vec3D;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color(pub Vec3D<f32>);

impl Color {
    const ONE_OVER_255: f32 = 1.0 / 255.0;
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color(Vec3D::new(
            Color::ONE_OVER_255 * (r as f32),
            Color::ONE_OVER_255 * (g as f32),
            Color::ONE_OVER_255 * (b as f32),
        ))
    }

    pub fn from_vec3d(color_vec: Vec3D<f32>) -> Color {
        Color(<Vec3D<f32>>::clamp(&color_vec, 0.0, 1.0))
    }

    pub fn from_hex_str(string: String) -> Color {
        use std::str;

        let hex_strings = &string[1..string.len()]
            .as_bytes()
            .chunks(2)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
            .collect::<Vec<&str>>();

        Color::new(
            u8::from_str_radix(hex_strings[0], 16).unwrap_or_default(),
            u8::from_str_radix(hex_strings[1], 16).unwrap_or_default(),
            u8::from_str_radix(hex_strings[2], 16).unwrap_or_default(),
        )
    }

    // --------------------

    pub fn r(&self) -> u8 {
        (self.0.x * 255.0) as u8
    }

    pub fn g(&self) -> u8 {
        (self.0.y * 255.0) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0.z * 255.0) as u8
    }

    // --------------------

    pub fn to_u32(self) -> u32 {
        (((Color::ONE_OVER_255 * self.0.x) as u32) << 16)
            | (((Color::ONE_OVER_255 * self.0.x) as u32) << 8)
            | ((Color::ONE_OVER_255 * self.0.x) as u32)
    }

    pub fn to_string(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::new(255, 255, 255)
    }
}
