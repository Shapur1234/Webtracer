use super::material::Material;
use super::object::ObjectList;
use crate::draw::Color;
use crate::vector::Vec3D;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// --------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ray {
    pub from: Vec3D<f32>,
    pub dir: Vec3D<f32>,
}

impl Ray {
    pub const fn new(from: Vec3D<f32>, dir: Vec3D<f32>) -> Ray {
        Ray { from, dir }
    }

    pub fn at(&self, dist: f32) -> Vec3D<f32> {
        self.from + self.dir * dist
    }

    pub fn ray_color(
        &self,
        object_list: &ObjectList,
        background_color: Option<Color>,
        depth: i32,
    ) -> Color {
        if depth <= 0 {
            Color::new(0, 0, 0)
        } else if let Some(record) = object_list.hit(self, Some(0.001), None) {
            let color_emmited = record.material.emmited(record.u, record.v, record.pos);
            if let Some((ray, attenuation)) = record.material.scatter(&record, self) {
                let ray_color = ray.ray_color(object_list, background_color, depth - 1);
                Color::from_vec3d(
                    color_emmited.0
                        + Vec3D::new(
                            attenuation.0.x * ray_color.0.x,
                            attenuation.0.y * ray_color.0.y,
                            attenuation.0.z * ray_color.0.z,
                        ),
                )
            } else {
                color_emmited
            }
        } else {
            background_color.unwrap_or(Ray::background_gradient(&self.dir))
        }
    }

    pub fn ray_color_simple(
        &self,
        object_list: &ObjectList,
        background_color: Option<Color>,
    ) -> Color {
        if let Some(record) = object_list.hit(self, None, None) {
            record.material.color(record.u, record.v, record.pos)
        } else {
            background_color.unwrap_or(Ray::background_gradient(&self.dir))
        }
    }

    fn background_gradient(ray_dir: &Vec3D<f32>) -> Color {
        let t = (ray_dir.y + 1.0) / 2.0;
        Color::from_vec3d(Vec3D::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3D::new(0.5, 0.7, 1.0) * t)
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RayHit {
    pub pos: Vec3D<f32>,
    pub normal: Vec3D<f32>,
    // pub dist: f32,
    pub front_face: bool,
    pub u: f32,
    pub v: f32,
    pub material: Arc<Material>,
}

impl RayHit {
    pub fn new(
        pos: Vec3D<f32>,
        normal: Vec3D<f32>,
        // dist: f32,
        front_face: bool,
        u: f32,
        v: f32,
        material: Arc<Material>,
    ) -> RayHit {
        RayHit {
            pos,
            // dist,
            normal: if front_face { normal } else { -normal },
            front_face,
            u,
            v,
            material,
        }
    }
}
