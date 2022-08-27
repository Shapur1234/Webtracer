use super::camera::Camera;
use super::object::ObjectList;
use super::ray::Ray;
use super::Object3D;

use crate::draw::{Color, FrameBuffer};
use crate::misc::cpu_cores;
use crate::vector::{Vec2D, Vec3D};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};

// --------------------------------------------------

pub static RENDER: Mutex<(bool, FrameBuffer)> =
    Mutex::new((false, FrameBuffer::new_dummy(Vec2D::new(0, 0))));

// --------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub object_list: ObjectList,
    pub background_color: Option<Color>,
    #[serde(skip)]
    pub size: Vec2D<u32>,
}

impl Scene {
    pub fn new(
        size: Vec2D<u32>,
        vfov: f32,
        background_color: Option<Color>,
        object_list: ObjectList,
    ) -> Scene {
        Scene {
            camera: Camera::new(
                Vec3D::new(0.0, 0.0, 0.0),
                Vec3D::new(0.0, 0.0, 1.0),
                (size.x as f32) / (size.y as f32),
                vfov,
            ),
            object_list,
            background_color,
            size,
        }
    }

    pub fn change_size(&mut self, size: Vec2D<u32>) {
        self.size = size;
        self.camera.resize((size.x as f32) / (size.y as f32));
    }

    pub fn get_pointed_at_object(&mut self) -> Option<&mut Object3D> {
        self.object_list.hit_object3d(
            &Ray::new(self.camera.pos, self.camera.pos + self.camera.rotation),
            None,
            None,
        )
    }

    pub fn get_pointed_at_object_index(&mut self) -> Option<usize> {
        if let Some(object) = self.object_list.clone().hit_object3d(
            &Ray::new(self.camera.pos, self.camera.pos + self.camera.rotation),
            None,
            None,
        ) {
            Some(
                self.object_list
                    .objects
                    .iter()
                    .position(|x| x == object)
                    .expect("Value should exist in self.object_list"),
            )
        } else {
            None
        }
    }

    pub fn render_full(&self, rays_per_pixel: u32, diffuse_bounce_depth: i32, all_cores: bool) {
        web_sys::console::time_with_label("Render start");

        let num_of_threads = if all_cores { cpu_cores() } else { 1 };
        let rays_per_pixel_per_thread = rays_per_pixel / num_of_threads;

        let working_threads = Arc::new(Mutex::new(num_of_threads));
        let out_color_vec: Arc<Mutex<Vec<Color>>> = Arc::new(Mutex::new(vec![]));
        let objects = Arc::new(
            self.object_list
                .camera_shifted(&self.camera)
                .camera_sorted(&self.camera),
        );

        let self_clone = Arc::new(self.clone());
        for _ in 0..num_of_threads {
            let self_clone = Arc::clone(&self_clone);
            let objects = Arc::clone(&objects);
            let working_threads_2 = Arc::clone(&working_threads);
            let out_color_vec_2 = Arc::clone(&out_color_vec);

            wasm_thread::spawn(move || {
                let render_output = self_clone.do_render(
                    rays_per_pixel_per_thread,
                    diffuse_bounce_depth,
                    &*objects,
                );

                {
                    let mut out_color_vec = out_color_vec_2.lock().unwrap();

                    *out_color_vec = if out_color_vec.is_empty() {
                        render_output
                    } else {
                        Scene::average_color_vecs(&*out_color_vec, &render_output)
                    }
                }
                {
                    let mut working_threads = working_threads_2.lock().unwrap();

                    *working_threads -= 1;
                    if *working_threads == 0 {
                        *RENDER.lock().unwrap() = (true, {
                            let mut frame_buffer = FrameBuffer::new(self_clone.size);
                            frame_buffer
                                .set_buffer(&*out_color_vec_2.lock().unwrap())
                                .unwrap();
                            frame_buffer
                        })
                    }
                }
            });
        }
    }

    pub fn render_simple(&self) -> FrameBuffer {
        let mut rng = thread_rng();

        // let size = self.size / 2;
        let mut frame_buffer = FrameBuffer::new(self.size);
        let objects_processed = self
            .object_list
            .camera_shifted(&self.camera)
            .camera_sorted(&self.camera);
        let size_minus_1 = Vec2D::new((self.size.x - 1) as f32, (self.size.y - 1) as f32);

        frame_buffer
            .set_buffer(
                &Scene::x_y_vec(self.size)
                    .into_iter()
                    .map(|pixel| {
                        self.camera
                            .get_ray(
                                (pixel.x + rng.gen::<f32>()) / size_minus_1.x,
                                (pixel.y + rng.gen::<f32>()) / size_minus_1.y,
                            )
                            .ray_color_simple(&objects_processed, self.background_color)
                    })
                    .collect(),
            )
            .unwrap();

        frame_buffer
    }

    fn do_render(
        &self,
        rays_per_pixel: u32,
        diffuse_bounce_depth: i32,
        object_list: &ObjectList,
    ) -> Vec<Color> {
        let mut rng = thread_rng();
        let size_minus_1 = Vec2D::new((self.size.x - 1) as f32, (self.size.y - 1) as f32);

        Scene::x_y_vec(self.size)
            .into_iter()
            .map(|pixel| {
                if pixel.x == 0.0 && pixel.y.rem_euclid(8.0) == 0.0 {
                    log::info!(
                        "{:?} - {:.3} %",
                        wasm_thread::current().id(),
                        100.0 - ((100.0 / (self.size.y as f32)) * pixel.y)
                    )
                }

                let mut color = Color::new(0, 0, 0);
                for _ in 0..rays_per_pixel {
                    color.0 += self
                        .camera
                        .get_ray(
                            (pixel.x + rng.gen::<f32>()) / size_minus_1.x,
                            (pixel.y + rng.gen::<f32>()) / size_minus_1.y,
                        )
                        .ray_color(object_list, self.background_color, diffuse_bounce_depth)
                        .0;
                }
                Scene::gamma_color_correct(color.0 / (rays_per_pixel as f32))
            })
            .collect::<Vec<Color>>()
    }

    fn x_y_vec(size: Vec2D<u32>) -> Vec<Vec2D<f32>> {
        let mut out = vec![];
        for y in 0..size.y {
            for x in 0..size.x {
                out.push(Vec2D::new(x as f32, (size.y as f32) - y as f32))
            }
        }
        out
    }
    fn gamma_color_correct(vec: Vec3D<f32>) -> Color {
        Color::from_vec3d(Vec3D::new((vec.x).sqrt(), (vec.y).sqrt(), (vec.z).sqrt()))
    }

    fn average_color_vecs(vec_1: &Vec<Color>, vec_2: &Vec<Color>) -> Vec<Color> {
        assert_eq!(vec_1.len(), vec_2.len(), "Vecs must be of same len");

        (0..vec_1.len())
            .into_iter()
            .map(|i| Color::from_vec3d((vec_1[i].0 + vec_2[i].0) / 2.0))
            .collect::<Vec<Color>>()
    }
}

// --------------------------------------------------

#[derive(Clone, Debug)]
pub struct SceneList {
    scenes: Vec<Scene>,
    current_scene: i32,
}

impl SceneList {
    pub fn new(
        object_lists: Vec<(ObjectList, Option<Color>)>,
        size: Vec2D<u32>,
        vfov: f32,
    ) -> SceneList {
        SceneList {
            scenes: object_lists
                .into_iter()
                .map(|object_list| Scene::new(size, vfov, object_list.1, object_list.0))
                .collect(),
            current_scene: 0,
        }
    }

    pub fn change_size(&mut self, new_size: Vec2D<u32>) {
        self.scenes
            .iter_mut()
            .for_each(|scene| scene.change_size(new_size));
    }

    pub fn change_vfov(&mut self, new_vfov: f32) {
        self.scenes
            .iter_mut()
            .for_each(|scene| scene.camera.change_vfov(new_vfov));
    }

    pub fn camera_reset(&mut self) {
        self.scenes
            .iter_mut()
            .for_each(|scene| scene.camera.reset());
    }

    fn clamp_current_scene(&mut self) {
        let len = self.scenes.len() as i32;
        if self.current_scene < 0 {
            self.current_scene = len - 1;
        } else if self.current_scene >= len {
            self.current_scene = 0;
        }
    }

    pub fn current_scene_index(&self) -> u32 {
        self.current_scene as u32
    }

    pub fn current(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene as usize]
    }

    pub fn current_immut(&self) -> &Scene {
        &self.scenes[self.current_scene as usize]
    }

    pub fn next(&mut self) {
        self.current_scene += 1;
        self.clamp_current_scene();
    }

    pub fn prev(&mut self) {
        self.current_scene -= 1;
        self.clamp_current_scene();
    }
}
