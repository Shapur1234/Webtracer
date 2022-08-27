use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    draw::Color,
    image::ImageData,
    vector::{Vec2D, Vec3D},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Texture {
    SolidColor {
        color: Color,
    },
    Checkered {
        odd_color: Color,
        even_color: Color,
        check_size: f32,
    },
    Image {
        data: Arc<ImageData>,
    },
}

impl Texture {
    pub fn color_value(&self, u: f32, v: f32, point: Vec3D<f32>) -> Color {
        match self {
            Texture::SolidColor { color } => *color,
            Texture::Checkered {
                odd_color,
                even_color,
                check_size,
            } => {
                if (check_size * point.x).sin()
                    * (check_size * point.y).sin()
                    * (check_size * point.z).sin()
                    < 0.0
                {
                    *odd_color
                } else {
                    *even_color
                }
            }
            Texture::Image { data } => data[Vec2D::new(u, 1.0 - v)],
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Texture::SolidColor {
            color: Color::default(),
        }
    }
}
