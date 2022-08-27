use std::cmp::Ordering;
use std::sync::Arc;

use super::camera::Camera;
use super::ray::{Ray, RayHit};
use super::Material;
use super::Texture;
use crate::vector::{Vec2D, Vec3D, VectorOperation};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Object3D {
    Sphere {
        pos: Vec3D<f32>,
        radius: f32,
        material: Arc<Material>,
    },
    Brick {
        pos: Vec3D<f32>,
        corner: Vec3D<f32>,
        sides: ObjectList,
        material: Arc<Material>,
    },
    XYRect {
        pos: Vec2D<f32>,
        corner: Vec2D<f32>,
        k: f32,
        material: Arc<Material>,
    },
    XZRect {
        pos: Vec2D<f32>,
        corner: Vec2D<f32>,
        k: f32,
        material: Arc<Material>,
    },
    YZRect {
        pos: Vec2D<f32>,
        corner: Vec2D<f32>,
        k: f32,
        material: Arc<Material>,
    },
}

impl Object3D {
    pub fn relative_posed(&self, camera: &Camera) -> Object3D {
        match self {
            Object3D::Sphere {
                pos,
                radius,
                material,
            } => Object3D::Sphere {
                pos: *pos - camera.pos,
                radius: *radius,
                material: material.clone(),
            },
            Object3D::Brick {
                pos,
                corner,
                sides,
                material,
            } => Object3D::Brick {
                pos: *pos - camera.pos,
                corner: *corner - camera.pos,
                sides: sides.camera_shifted(camera),
                material: material.clone(),
            },
            Object3D::XYRect {
                pos,
                corner,
                k,
                material,
            } => {
                let cam_pos = Vec2D::new(camera.pos.x, camera.pos.y);
                Object3D::XYRect {
                    pos: *pos - cam_pos,
                    corner: *corner - cam_pos,
                    k: *k - camera.pos.z,
                    material: material.clone(),
                }
            }
            Object3D::XZRect {
                pos,
                corner,
                k,
                material,
            } => {
                let cam_pos = Vec2D::new(camera.pos.x, camera.pos.z);
                Object3D::XZRect {
                    pos: *pos - cam_pos,
                    corner: *corner - cam_pos,
                    k: *k - camera.pos.y,
                    material: material.clone(),
                }
            }
            Object3D::YZRect {
                pos,
                corner,
                k,
                material,
            } => {
                let cam_pos = Vec2D::new(camera.pos.y, camera.pos.z);
                Object3D::YZRect {
                    pos: *pos - cam_pos,
                    corner: *corner - cam_pos,
                    k: *k - camera.pos.x,
                    material: material.clone(),
                }
            }
        }
    }

    pub fn brick(pos: Vec3D<f32>, size: Vec3D<f32>, material: Arc<Material>) -> Object3D {
        Object3D::Brick {
            pos,
            corner: pos + size,
            sides: ObjectList::new(vec![
                Object3D::XYRect {
                    pos: Vec2D::new(pos.x, pos.y),
                    corner: Vec2D::new(pos.x + size.x, pos.y + size.y),
                    k: pos.z,
                    material: material.clone(),
                },
                Object3D::XYRect {
                    pos: Vec2D::new(pos.x, pos.y),
                    corner: Vec2D::new(pos.x + size.x, pos.y + size.y),
                    k: pos.z + size.z,
                    material: material.clone(),
                },
                Object3D::YZRect {
                    pos: Vec2D::new(pos.y, pos.z),
                    corner: Vec2D::new(pos.y + size.y, pos.z + size.z),
                    k: pos.x,
                    material: material.clone(),
                },
                Object3D::YZRect {
                    pos: Vec2D::new(pos.y, pos.z),
                    corner: Vec2D::new(pos.y + size.y, pos.z + size.z),
                    k: pos.x + size.x,
                    material: material.clone(),
                },
                Object3D::XZRect {
                    pos: Vec2D::new(pos.x, pos.z),
                    corner: Vec2D::new(pos.x + size.x, pos.z + size.z),
                    k: pos.y,
                    material: material.clone(),
                },
                Object3D::XZRect {
                    pos: Vec2D::new(pos.x, pos.z),
                    corner: Vec2D::new(pos.x + size.x, pos.z + size.z),
                    k: pos.y + size.y,
                    material: material.clone(),
                },
            ]),
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, dist_min: Option<f32>, dist_max: Option<f32>) -> Option<RayHit> {
        const DIST_MIN_DEFAULT: f32 = 0.0;
        const DIST_MAX_DEFAULT: f32 = f32::INFINITY;

        match self {
            Object3D::Sphere {
                pos,
                radius,
                material,
            } => {
                let oc = ray.from - *pos;

                let a = ray.dir.length_squared();
                let half_b = oc.dot(&ray.dir);
                let c = oc.length_squared() - radius.powf(2.0);
                let discriminant = half_b.powf(2.0) - a * c;

                if discriminant > 0.0 {
                    let discriminant_sqrt = discriminant.powf(0.5);
                    let mut root = (-half_b - discriminant_sqrt) / a;

                    if root < dist_min.unwrap_or(DIST_MIN_DEFAULT)
                        || root > dist_max.unwrap_or(DIST_MAX_DEFAULT)
                    {
                        root = (-half_b + discriminant_sqrt) / a;
                        if root < dist_min.unwrap_or(DIST_MIN_DEFAULT)
                            || root > dist_max.unwrap_or(DIST_MAX_DEFAULT)
                        {
                            return None;
                        }
                    }
                    Some({
                        let ray_hit_pos = ray.at(root);

                        let d = (ray_hit_pos - *pos).unit_vec();
                        let (u, v) = if let Some(texture) = material.texture() {
                            if !matches!(texture, Texture::SolidColor { .. }) {
                                (
                                    0.5 + (((d.x).atan2(d.z)) / (2.0 * std::f32::consts::PI)),
                                    0.5 + (((d.y).asin()) / (std::f32::consts::PI)),
                                )
                            } else {
                                (0.0, 0.0)
                            }
                        } else {
                            (0.0, 0.0)
                        };

                        let at_root = ray.at(root);
                        RayHit::new(
                            ray_hit_pos,
                            (at_root - *pos) / *radius,
                            // at_root.dist_between(&ray.from),
                            ray.dir.dot(&((ray_hit_pos - *pos) / *radius)) < 0.0,
                            u,
                            v,
                            material.clone(),
                        )
                    })
                } else {
                    None
                }
            }
            Object3D::XYRect {
                pos,
                corner,
                k,
                material,
            } => {
                let dist = (k - ray.from.z) / ray.dir.z;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (x, y) = (ray.from.x + dist * ray.dir.x, ray.from.y + dist * ray.dir.y);
                    if x > pos.x && x < corner.x && y > pos.y && y < corner.y {
                        let outwards_normal = Vec3D::new(0.0, 0.0, 1.0);
                        let at_dist = ray.at(dist);

                        Some(RayHit::new(
                            at_dist,
                            outwards_normal,
                            // at_dist.dist_between(&ray.from),
                            ray.dir.dot(&outwards_normal) < 0.0,
                            (x - pos.x) / (corner.x - pos.x),
                            (y - pos.y) / (corner.y - pos.y),
                            material.clone(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::XZRect {
                pos,
                corner,
                k,
                material,
            } => {
                let dist = (k - ray.from.y) / ray.dir.y;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (x, z) = (ray.from.x + dist * ray.dir.x, ray.from.z + dist * ray.dir.z);
                    if x > pos.x && x < corner.x && z > pos.y && z < corner.y {
                        let outwards_normal = Vec3D::new(0.0, 1.0, 0.0);
                        let at_dist = ray.at(dist);

                        Some(RayHit::new(
                            at_dist,
                            outwards_normal,
                            // at_dist.dist_between(&ray.from),
                            ray.dir.dot(&outwards_normal) < 0.0,
                            (x - pos.x) / (corner.x - pos.x),
                            (z - pos.y) / (corner.y - pos.y),
                            material.clone(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::YZRect {
                pos,
                corner,
                k,
                material,
            } => {
                let dist = (k - ray.from.x) / ray.dir.x;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (y, z) = (ray.from.y + dist * ray.dir.y, ray.from.z + dist * ray.dir.z);
                    if y > pos.x && y < corner.x && z > pos.y && z < corner.y {
                        let outwards_normal = Vec3D::new(1.0, 0.0, 0.0);
                        let at_dist = ray.at(dist);

                        Some(RayHit::new(
                            at_dist,
                            outwards_normal,
                            // at_dist.dist_between(&ray.from),
                            ray.dir.dot(&outwards_normal) < 0.0,
                            (y - pos.x) / (corner.x - pos.x),
                            (z - pos.y) / (corner.y - pos.y),
                            material.clone(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::Brick { sides, .. } => sides.hit(ray, dist_min, dist_max),
        }
    }

    pub fn hit_object3d(
        &mut self,
        ray: &Ray,
        dist_min: Option<f32>,
        dist_max: Option<f32>,
    ) -> Option<&mut Object3D> {
        const DIST_MIN_DEFAULT: f32 = 0.0;
        const DIST_MAX_DEFAULT: f32 = f32::INFINITY;

        match self {
            Object3D::Sphere { pos, radius, .. } => {
                let oc = ray.from - *pos;

                let a = ray.dir.length_squared();
                let half_b = oc.dot(&ray.dir);
                let c = oc.length_squared() - radius.powf(2.0);
                let discriminant = half_b.powf(2.0) - a * c;

                if discriminant > 0.0 {
                    let discriminant_sqrt = discriminant.powf(0.5);
                    let mut root = (-half_b - discriminant_sqrt) / a;

                    if root < dist_min.unwrap_or(DIST_MIN_DEFAULT)
                        || root > dist_max.unwrap_or(DIST_MAX_DEFAULT)
                    {
                        root = (-half_b + discriminant_sqrt) / a;
                        if root < dist_min.unwrap_or(DIST_MIN_DEFAULT)
                            || root > dist_max.unwrap_or(DIST_MAX_DEFAULT)
                        {
                            return None;
                        }
                    }
                    Some(self)
                } else {
                    None
                }
            }
            Object3D::XYRect { pos, corner, k, .. } => {
                let dist = (*k - ray.from.z) / ray.dir.z;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (x, y) = (ray.from.x + dist * ray.dir.x, ray.from.y + dist * ray.dir.y);
                    if x > pos.x && x < corner.x && y > pos.y && y < corner.y {
                        Some(self)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::XZRect { pos, corner, k, .. } => {
                let dist = (*k - ray.from.y) / ray.dir.y;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (x, z) = (ray.from.x + dist * ray.dir.x, ray.from.z + dist * ray.dir.z);
                    if x > pos.x && x < corner.x && z > pos.y && z < corner.y {
                        Some(self)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::YZRect { pos, corner, k, .. } => {
                let dist = (*k - ray.from.x) / ray.dir.x;
                if dist > dist_min.unwrap_or(DIST_MIN_DEFAULT)
                    && dist < dist_max.unwrap_or(DIST_MAX_DEFAULT)
                {
                    let (y, z) = (ray.from.y + dist * ray.dir.y, ray.from.z + dist * ray.dir.z);
                    if y > pos.x && y < corner.x && z > pos.y && z < corner.y {
                        Some(self)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Object3D::Brick { sides, .. } => {
                if sides.hit_object3d(ray, None, None).is_some() {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    pub fn distance_from_camera(&self, cam: &Camera) -> f32 {
        match self {
            Object3D::Sphere { pos, .. } => ((pos.x - cam.pos.x).powf(2.0)
                + (pos.y - cam.pos.y).powf(2.0)
                + (pos.z - cam.pos.z).powf(2.0))
            .powf(0.5),
            Object3D::Brick { pos, corner, .. } => {
                let pos = (*pos + *corner) / 2.0;
                (pos.x - cam.pos.x).powf(2.0)
                    + (pos.y - cam.pos.y).powf(2.0)
                    + (pos.z - cam.pos.z).powf(2.0)
            }
            Object3D::XYRect { pos, corner, k, .. } => {
                let pos = (Vec3D::new(pos.x, pos.y, *k) + Vec3D::new(corner.x, corner.y, *k)) / 2.0;
                (pos.x - cam.pos.x).powf(2.0)
                    + (pos.y - cam.pos.y).powf(2.0)
                    + (pos.z - cam.pos.z).powf(2.0)
            }
            Object3D::XZRect { pos, corner, k, .. } => {
                let pos = (Vec3D::new(pos.x, *k, pos.y) + Vec3D::new(corner.x, *k, corner.y)) / 2.0;
                (pos.x - cam.pos.x).powf(2.0)
                    + (pos.y - cam.pos.y).powf(2.0)
                    + (pos.z - cam.pos.z).powf(2.0)
            }
            Object3D::YZRect { pos, corner, k, .. } => {
                let pos = (Vec3D::new(*k, pos.x, pos.y) + Vec3D::new(*k, corner.x, corner.y)) / 2.0;
                (pos.x - cam.pos.x).powf(2.0)
                    + (pos.y - cam.pos.y).powf(2.0)
                    + (pos.z - cam.pos.z).powf(2.0)
            }
        }
    }
}

// --------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ObjectList {
    pub objects: Vec<Object3D>,
}

impl ObjectList {
    pub const fn new(objects: Vec<Object3D>) -> ObjectList {
        ObjectList { objects }
    }

    pub fn camera_sorted(&self, camera: &Camera) -> ObjectList {
        ObjectList::new({
            let mut out = self.objects.clone();
            out.sort_by(|object1, object2| {
                let (dist1, dist2) = (
                    object1.distance_from_camera(camera),
                    object2.distance_from_camera(camera),
                );

                if dist1 < dist2 {
                    Ordering::Less
                } else if dist1 > dist2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            out.iter_mut().for_each(|ref mut x| match x {
                Object3D::Brick { sides, .. } => {
                    *sides = sides.camera_sorted(camera);
                }
                _ => {}
            });
            out
        })
    }

    pub fn camera_shifted(&self, camera: &Camera) -> ObjectList {
        let mut out: Vec<Object3D> = self
            .objects
            .iter()
            .map(|object| object.relative_posed(camera))
            .collect();
        out.iter_mut().for_each(|ref mut x| match x {
            Object3D::Brick { sides, .. } => {
                *sides = sides.camera_shifted(camera);
            }
            _ => {}
        });
        ObjectList::new(out)
    }

    pub fn hit(&self, ray: &Ray, dist_min: Option<f32>, dist_max: Option<f32>) -> Option<RayHit> {
        for object in &self.objects {
            if let Some(record) = object.hit(ray, dist_min, dist_max) {
                return Some(record);
            }
        }
        None
    }

    pub fn hit_object3d(
        &mut self,
        ray: &Ray,
        dist_min: Option<f32>,
        dist_max: Option<f32>,
    ) -> Option<&mut Object3D> {
        for object in &mut self.objects {
            if let Some(object) = object.hit_object3d(ray, dist_min, dist_max) {
                return Some(object);
            }
        }
        None
    }

    // pub fn hit_object3d(
    //     &mut self,
    //     ray: &Ray,
    //     dist_min: Option<f32>,
    //     dist_max: Option<f32>,
    // ) -> Option<&mut Object3D> {
    //     let mut closest_found: Option<RayHit> = None;
    //     let mut out = None;
    //
    //     for object in &mut self.objects {
    //         if let Some(record) = object.hit(ray, dist_min, dist_max) {
    //             if let Some(closest) = &mut closest_found {
    //                 if record.dist < closest.dist {
    //                     closest_found = Some(record);
    //                 }
    //             } else {
    //                 closest_found = Some(record);
    //             }
    //             out = object.hit_object3d(ray, dist_min, dist_max);
    //         }
    //     }
    //     out
    // }
}
