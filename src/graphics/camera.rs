use super::ray::Ray;
use crate::vector::{Vec2D, Vec3D, VectorOperation};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CameraInput {
    pub forward: bool,
    pub back: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub reset: bool,
    pub mouse_move: Option<Vec2D<f32>>,
}

impl CameraInput {
    pub fn reset(&mut self) {
        *self = CameraInput::default();
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub pos: Vec3D<f32>,
    pub rotation: Vec3D<f32>,
    pub vfov: f32,
    #[serde(skip)]
    aspect_ratio: f32,
    #[serde(skip)]
    horizontal: Vec3D<f32>,
    #[serde(skip)]
    vertical: Vec3D<f32>,
    #[serde(skip)]
    lower_left_corner: Vec3D<f32>,
}

impl Camera {
    pub fn new(pos: Vec3D<f32>, rotation: Vec3D<f32>, aspect_ratio: f32, vfov: f32) -> Camera {
        let w = (-rotation).unit_vec();
        let u = Vec3D::new(0.0, 1.0, 0.0).cross(&w);
        let v = w.cross(&u);

        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        Camera {
            pos,
            rotation,
            vfov,
            aspect_ratio,
            horizontal,
            vertical,
            lower_left_corner: pos - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn change_vfov(&mut self, vfov: f32) {
        *self = Camera::new(self.pos, self.rotation, self.aspect_ratio, vfov)
    }

    pub fn reset(&mut self) {
        *self = Camera::new(
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 1.0),
            self.aspect_ratio,
            self.vfov,
        )
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        *self = Camera::new(self.pos, self.rotation, aspect_ratio, self.vfov)
    }

    fn update(&mut self) {
        *self = Camera::new(self.pos, self.rotation, self.aspect_ratio, self.vfov)
    }

    pub fn handle_input(&mut self, input: &CameraInput) {
        const ROTATION_SPEED_MULTIPLIER: f32 = 0.001;
        const MOVEMENT_SPEED_MULTIPLIER: f32 = 0.05;

        if input.reset {
            return self.reset();
        }

        if input.forward {
            self.pos += self.rotation * MOVEMENT_SPEED_MULTIPLIER
        }
        if input.back {
            self.pos -= self.rotation * MOVEMENT_SPEED_MULTIPLIER
        }
        if input.right {
            self.pos -= self.rotation.rotate_around_y(std::f32::consts::PI / 2.0)
                * MOVEMENT_SPEED_MULTIPLIER
        }
        if input.left {
            self.pos += self.rotation.rotate_around_y(std::f32::consts::PI / 2.0)
                * MOVEMENT_SPEED_MULTIPLIER
        }
        if input.up {
            self.pos.y += self.rotation.length() * MOVEMENT_SPEED_MULTIPLIER
        }
        if input.down {
            self.pos.y -= self.rotation.length() * MOVEMENT_SPEED_MULTIPLIER
        }

        if let Some(mouse_move) = input.mouse_move {
            let rotate = Vec2D::new(
                -mouse_move.x,
                mouse_move.y * if self.rotation.z > 0.0 { 1.0 } else { -1.0 },
            ) * ROTATION_SPEED_MULTIPLIER;

            self.rotation = self.rotation.rotate_around_y(rotate.x);
            self.rotation = self.rotation.rotate_around_x(rotate.y);
        }

        self.update();
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(
            self.pos,
            self.lower_left_corner + self.horizontal * x + self.vertical * y - self.pos,
        )
    }
}
