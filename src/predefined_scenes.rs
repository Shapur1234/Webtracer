use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::draw::Color;
use crate::graphics::{Material, Object3D, ObjectList, Texture};
use crate::image::{get_const_image, ImageID};
use crate::vector::Vec3D;

pub fn predefined_scenes() -> Vec<(ObjectList, Option<Color>)> {
    vec![
        {
            let material_ground = Arc::new(Material::Lambertian {
                texture: Texture::Checkered {
                    odd_color: Color::new(5, 55, 123),
                    even_color: Color::new(3, 33, 74),
                    check_size: 20.0,
                },
            });
            let material_center = Arc::new(Material::Metal {
                fuzz: 0.1,
                texture: Texture::Image {
                    data: get_const_image(ImageID::Earth),
                },
            });
            let material_right = Arc::new(Material::Lambertian {
                texture: Texture::Image {
                    data: get_const_image(ImageID::Mars),
                },
            });
            let material_left = Arc::new(Material::Lambertian {
                texture: Texture::Image {
                    data: get_const_image(ImageID::Jupiter),
                },
            });
            let material_light = Arc::new(Material::DiffuseLight {
                texture: Texture::Image {
                    data: get_const_image(ImageID::Sun),
                },
            });

            (
                ObjectList::new(vec![
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, 4.0, 1.0),
                        radius: 2.0,
                        material: material_light,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, 0.6, 1.0),
                        radius: 0.5,
                        material: material_center,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(-1.2, 0.5, 1.0),
                        radius: 0.5,
                        material: material_right,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(1.2, 0.5, 1.0),
                        radius: 0.5,
                        material: material_left,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, -1000.1, 1.0),
                        radius: 1000.0,
                        material: material_ground,
                    },
                ]),
                Some(Color::new(0, 0, 0)),
            )
        },
        {
            let material_center = Arc::new(Material::Lambertian {
                texture: Texture::Image {
                    data: get_const_image(ImageID::Brick),
                },
            });
            let material_light = Arc::new(Material::DiffuseLight {
                texture: Texture::SolidColor {
                    color: Color::new(255, 255, 255),
                },
            });
            let material_ground = Arc::new(Material::Lambertian {
                texture: Texture::Checkered {
                    odd_color: Color::new(200, 200, 200),
                    even_color: Color::new(100, 100, 100),
                    check_size: 10.0,
                },
            });

            (
                ObjectList::new(vec![
                    Object3D::brick(
                        Vec3D::new(-2.0, 0.2, 1.4),
                        Vec3D::new(0.02, 1.0, 0.8),
                        material_light,
                    ),
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, 0.5, 1.0),
                        radius: 0.5,
                        material: material_center,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, -1000.1, 0.0),
                        radius: 1000.0,
                        material: material_ground,
                    },
                ]),
                Some(Color::new(0, 0, 0)),
            )
        },
        {
            let white = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(200, 200, 200),
                },
            });
            let grey = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(150, 150, 150),
                },
            });
            let red = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(255, 80, 80),
                },
            });
            let green = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(30, 114, 38),
                },
            });
            let light = Arc::new(Material::DiffuseLight {
                texture: Texture::SolidColor {
                    color: Color::new(255, 255, 255),
                },
            });

            (
                ObjectList::new(vec![
                    Object3D::brick(
                        Vec3D::new(-2.0, -1.0, -2.0),
                        Vec3D::new(4.0, -0.05, 4.0),
                        white.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-2.0, 3.0, -2.0),
                        Vec3D::new(4.0, 0.05, 4.0),
                        white.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-2.0, -1.0, 2.0),
                        Vec3D::new(4.0, 4.0, 0.05),
                        white.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-2.0, -1.0, -2.0),
                        Vec3D::new(4.0, 4.0, -0.05),
                        white.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(2.0, -1.0, -2.0),
                        Vec3D::new(0.05, 4.0, 4.0),
                        green.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-2.0, -1.0, -2.0),
                        Vec3D::new(0.05, 4.0, 4.0),
                        red.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-0.8, 2.95, -0.8),
                        Vec3D::new(1.6, 0.05, 1.6),
                        light.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(0.8, -0.8, 1.2),
                        Vec3D::new(0.8, 2.0, 0.8),
                        grey.clone(),
                    ),
                    Object3D::brick(
                        Vec3D::new(-1.1, -0.8, 0.5),
                        Vec3D::new(1.2, 1.2, 1.2),
                        grey.clone(),
                    ),
                ]),
                Some(Color::new(0, 0, 0)),
            )
        },
        {
            let material_ground = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(0, 255, 0),
                },
            });
            let center = Arc::new(Material::Metal {
                texture: Texture::SolidColor {
                    color: Color::new(191, 191, 191),
                },
                fuzz: 0.2,
            });
            let upper_center = Arc::new(Material::Metal {
                texture: Texture::SolidColor {
                    color: Color::new(63, 63, 63),
                },
                fuzz: 0.5,
            });
            let material_row1 = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(205, 0, 26),
                },
            });
            let material_row2 = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(143, 0, 255),
                },
            });

            (
                ObjectList::new(vec![
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, 0.0, 1.0),
                        radius: 0.8,
                        material: center,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(-2.3, 0.05, 1.0),
                        radius: 0.4,
                        material: material_row1.clone(),
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(2.3, 0.05, 1.0),
                        radius: 0.4,
                        material: material_row1,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, 3.0, 1.0),
                        radius: 1.0,
                        material: upper_center,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(-2.3, 2.45, 1.0),
                        radius: 0.4,
                        material: material_row2.clone(),
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(2.3, 2.45, 1.0),
                        radius: 0.4,
                        material: material_row2,
                    },
                    Object3D::Sphere {
                        pos: Vec3D::new(0.0, -1000.1, 1.0),
                        radius: 1000.0,
                        material: material_ground,
                    },
                ]),
                Some(Color::new(254, 212, 140)),
            )
        },
        {
            let mut rng = thread_rng();

            let material_ground = Arc::new(Material::Lambertian {
                texture: Texture::SolidColor {
                    color: Color::new(124, 252, 0),
                },
            });

            let rand_materials = (0..20)
                .into_iter()
                .map(|_| {
                    Arc::new({
                        let material_choice = rng.gen::<f32>();
                        if material_choice < 0.5 {
                            Material::Lambertian {
                                texture: Texture::SolidColor {
                                    color: Color::from_vec3d(Vec3D::new_rand_between(0.2, 1.0)),
                                },
                            }
                        } else if material_choice < 0.9 {
                            Material::Metal {
                                texture: Texture::SolidColor {
                                    color: Color::from_vec3d(Vec3D::new_rand_between(0.2, 1.0)),
                                },
                                fuzz: rng.gen_range(0.0..=0.5),
                            }
                        } else {
                            Material::Dielectric {
                                refraction_index: 0.0,
                            }
                        }
                    })
                })
                .collect::<Vec<Arc<Material>>>();

            (
                ObjectList::new({
                    let mut out = vec![];

                    for x in -10..10 {
                        for y in -10..10 {
                            if x != 0 && y != 0 {
                                out.push({
                                    let radius = rng.gen_range(0.1..=0.4);
                                    Object3D::Sphere {
                                        pos: Vec3D::new(x as f32, radius, y as f32),
                                        radius,
                                        material: rand_materials[rng
                                            .gen_range(0.0..=(rand_materials.len() - 1) as f32)
                                            as usize]
                                            .clone(),
                                    }
                                });

                                // if x % 3 == 0 && y % 3 == 0 {
                                //     out.push({
                                //         let radius = rng.gen_range(0.1..=0.4);
                                //         Object3D::Sphere {
                                //             pos: Vec3D::new(x as f32, radius + 1.5, y as f32),
                                //             radius,
                                //             material: Arc::new(Material::DiffuseLight {
                                //                 texture: Texture::SolidColor {
                                //                     color: Color::new(
                                //                         rng.gen_range(200..=255),
                                //                         rng.gen_range(200..=255),
                                //                         rng.gen_range(200..=255),
                                //                     ),
                                //                 },
                                //             }),
                                //         }
                                //     })
                                // }
                            }
                        }
                    }
                    out.push(Object3D::Sphere {
                        pos: Vec3D::new(0.0, -1000.1, 0.0),
                        radius: 1000.0,
                        material: material_ground,
                    });
                    out
                }),
                None,
            )
        },
        // {
        //     let mut rng = thread_rng();
        //
        //     let material_ground = Arc::new(Material::Lambertian {
        //         texture: Texture::SolidColor {
        //             color: Color::new(122, 211, 135),
        //         },
        //     });
        //     let material_light = Arc::new(Material::DiffuseLight {
        //         texture: Texture::SolidColor {
        //             color: Color::new(255, 255, 255),
        //         },
        //     });
        //     let material_brown = Arc::new(Material::Lambertian {
        //         texture: Texture::SolidColor {
        //             color: Color::new(178, 76, 25),
        //         },
        //     });
        //     let material_earth = Arc::new(Material::Lambertian {
        //         texture: Texture::Image {
        //             data: get_const_image(ImageID::Earth),
        //         },
        //     });
        //
        //     (
        //         ObjectList::new({
        //             let mut out = vec![];
        //
        //             for x in -8..=8 {
        //                 for z in -8..=8 {
        //                     out.push(Object3D::brick(
        //                         Vec3D::new(
        //                             x as f32 - 0.5,
        //                             -1.0 + rng.gen_range(0.0..=0.5),
        //                             z as f32,
        //                         ),
        //                         Vec3D::new(1.0, 1.0, 1.0),
        //                         material_ground.clone(),
        //                     ))
        //                 }
        //             }
        //
        //             out.push(Object3D::brick(
        //                 Vec3D::new(-5.0, 8.0, -5.0),
        //                 Vec3D::new(10.0, 0.05, 10.0),
        //                 material_light,
        //             ));
        //
        //             out.push(Object3D::Sphere {
        //                 pos: Vec3D::new(4.0, 6.0, 6.0),
        //                 radius: 1.5,
        //                 material: material_brown,
        //             });
        //
        //             out.push(Object3D::Sphere {
        //                 pos: Vec3D::new(2.0, 5.0, 4.0),
        //                 radius: 1.2,
        //                 material: material_earth,
        //             });
        //
        //             out
        //         }),
        //         Some(Color::new(0, 0, 0)),
        //         // None,
        //     )
        // },
        { (ObjectList::new(vec![]), None) },
        { (ObjectList::new(vec![]), Some(Color::new(0, 0, 0))) },
    ]
}
