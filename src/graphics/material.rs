use super::ray::{Ray, RayHit};
use super::texture::Texture;
use crate::draw::Color;
use crate::vector::{Vec3D, VectorOperation};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Material {
    Lambertian { texture: Texture },
    Metal { texture: Texture, fuzz: f32 },
    Dielectric { refraction_index: f32 },
    DiffuseLight { texture: Texture },
}

impl Material {
    pub fn scatter(&self, record: &RayHit, ray_in: &Ray) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian { texture } => Some((
                Ray::new(
                    record.pos,
                    record.normal + Vec3D::new_rand_in_unit_sphere().unit_vec(),
                ),
                texture.color_value(record.u, record.v, record.pos),
            )),
            Material::Metal { texture, fuzz } => {
                let scattered = Ray::new(
                    record.pos,
                    Material::reflect(ray_in.dir, record.normal).unit_vec()
                        + Vec3D::new_rand_in_unit_sphere() * *fuzz,
                );

                if scattered.dir.dot(&record.normal) > 0.0 {
                    Some((
                        scattered,
                        texture.color_value(record.u, record.v, record.pos),
                    ))
                } else {
                    None
                }
            }
            Material::Dielectric { refraction_index } => {
                let refraction_ratio = if record.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };
                let unit_direction = ray_in.dir.unit_vec();

                let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powf(2.0)).powf(0.5);

                let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
                let direction = if cannot_refract
                    || Material::reflectance(cos_theta, refraction_ratio) > thread_rng().gen()
                {
                    Material::reflect(unit_direction, record.normal)
                } else {
                    Material::refract(unit_direction, record.normal, refraction_ratio)
                };

                Some((Ray::new(record.pos, direction), Color::default()))
            }
            Material::DiffuseLight { texture: _ } => None,
        }
    }

    pub fn emmited(&self, u: f32, v: f32, point: Vec3D<f32>) -> Color {
        match self {
            Material::Lambertian { texture: _ } => Color::new(0, 0, 0),
            Material::Metal {
                texture: _,
                fuzz: _,
            } => Color::new(0, 0, 0),
            Material::Dielectric {
                refraction_index: _,
            } => Color::new(0, 0, 0),
            Material::DiffuseLight { texture } => texture.color_value(u, v, point),
        }
    }

    pub fn color(&self, u: f32, v: f32, point: Vec3D<f32>) -> Color {
        match self {
            Material::Lambertian { texture } => texture.color_value(u, v, point),
            Material::Metal {
                texture,
                fuzz: _fuzz,
            } => texture.color_value(u, v, point),
            Material::DiffuseLight { texture } => texture.color_value(u, v, point),
            Material::Dielectric {
                refraction_index: _,
            } => Color::new(255, 255, 255),
        }
    }

    pub fn texture(&self) -> Option<&Texture> {
        match self {
            Material::Metal { texture, .. } => Some(texture),
            Material::Lambertian { texture } => Some(texture),
            Material::DiffuseLight { texture } => Some(texture),
            _ => None,
        }
    }

    fn reflect(v: Vec3D<f32>, n: Vec3D<f32>) -> Vec3D<f32> {
        v - n * 2.0 * v.dot(&n)
    }

    fn refract(uv: Vec3D<f32>, n: Vec3D<f32>, etai_over_etat: f32) -> Vec3D<f32> {
        let cos_theta = (-uv).dot(&n).min(1.0);
        let r_out_perp = (uv + (n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n * -(((1.0 - r_out_perp.length_squared()).abs()).powf(0.5));

        r_out_perp + r_out_parallel
    }

    fn reflectance(cos: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            texture: Texture::default(),
        }
    }
}
