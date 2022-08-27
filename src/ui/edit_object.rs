use std::collections::HashMap;
use std::sync::Arc;

use crate::draw::Color;
use crate::image::{get_const_image, ImageID};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};

use crate::graphics::{Material, Object3D, Texture};
use crate::misc::get_element_by_id;
use crate::vector::Vec3D;

// --------------------------------------------------

const POS: Vec3D<f64> = Vec3D::new(0.0, 0.0, 0.0);
const SIZE: f64 = 1.0;
const RADIUS: f64 = 1.0;
const FUZZ: f64 = 0.0;
const REFRACTION_INDEX: f64 = 0.0;
const CHECK_SIZE: f64 = 10.0;

// --------------------------------------------------

#[derive(Clone, Debug)]
pub struct EditObject {
    pub panel_element: HtmlElement,
    pub subsection_elements: HashMap<String, HtmlElement>,
    pub select_elements: HashMap<String, HtmlSelectElement>,
    pub input_elements: HashMap<String, HtmlInputElement>,
}

impl EditObject {
    pub fn hide(&self) {
        self.panel_element
            .style()
            .set_property("display", "none")
            .unwrap();
    }

    fn hide_sub_lines(&self) {
        for elem in &self.subsection_elements {
            if elem.0 != "object_multi_box" && elem.0 != "material_multi_box" {
                elem.1.style().set_property("display", "none").unwrap()
            }
        }
    }

    fn update_sub_visibility(&self, object: String, material: String) {
        self.hide_sub_lines();

        match object.as_str() {
            "sphere" => {
                self.subsection_elements["object_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["object_type_select"].set_value("sphere");
                self.subsection_elements["line_object_radius"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            "brick" => {
                self.subsection_elements["object_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["object_type_select"].set_value("brick");
                self.subsection_elements["line_object_size"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            _ => unreachable!(),
        }

        match material.as_str() {
            "lambertian" => {
                self.subsection_elements["material_multi_box"]
                    .style()
                    .set_property("height", "10%")
                    .unwrap();

                self.select_elements["material_select"].set_value("lambertian");
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            "metal" => {
                self.subsection_elements["material_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["material_select"].set_value("metal");
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.subsection_elements["line_material_fuzz"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            "dielectric" => {
                self.subsection_elements["material_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["material_select"].set_value("dielectric");
                self.subsection_elements["line_material_refraction_index"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            "diffuse_light" => {
                self.subsection_elements["material_multi_box"]
                    .style()
                    .set_property("height", "10%")
                    .unwrap();

                self.select_elements["material_select"].set_value("diffuse_light");
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
            }
            _ => unreachable!(),
        }
        // Haks
        self.set_texture(&self.get_texture());
    }

    pub fn update(&self) {
        for elem in &self.input_elements {
            match elem.0.as_str() {
                "pos_x" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(-1000000.0, 1000000.0)),
                    Err(_) => elem.1.set_value_as_number(POS.x),
                },
                "pos_y" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(-1000000.0, 1000000.0)),
                    Err(_) => elem.1.set_value_as_number(POS.y),
                },
                "pos_z" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(-10000.0, 10000.0)),
                    Err(_) => elem.1.set_value_as_number(POS.z),
                },
                "size_x" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.001, 10000.0)),
                    Err(_) => elem.1.set_value_as_number(SIZE),
                },
                "size_y" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.001, 10000.0)),
                    Err(_) => elem.1.set_value_as_number(SIZE),
                },
                "size_z" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.001, 10000.0)),
                    Err(_) => elem.1.set_value_as_number(SIZE),
                },
                "object_radius" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.001, 1000000.0)),
                    Err(_) => elem.1.set_value_as_number(RADIUS),
                },
                "material_fuzz" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.0, 1.0)),
                    Err(_) => elem.1.set_value_as_number(FUZZ),
                },
                "material_refraction_index" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(-1.0, 1.0)),
                    Err(_) => elem.1.set_value_as_number(REFRACTION_INDEX),
                },
                "texture_color" => {}
                "texture_color_odd" => {}
                "texture_color_even" => {}
                "texture_check_size" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(0.5, 100.0)),
                    Err(_) => elem.1.set_value_as_number(CHECK_SIZE),
                },
                _ => unreachable!(),
            }
        }
        self.update_sub_visibility(
            self.select_elements["object_type_select"].value(),
            self.select_elements["material_select"].value(),
        );
    }

    fn get_object(&self) -> Object3D {
        match self.select_elements["object_type_select"].value().as_str() {
            "sphere" => Object3D::Sphere {
                pos: Vec3D::new(
                    self.input_elements["pos_x"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.x as f32),
                    self.input_elements["pos_y"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.y as f32),
                    self.input_elements["pos_z"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.z as f32),
                ),
                radius: self.input_elements["object_radius"]
                    .value()
                    .parse::<f32>()
                    .unwrap_or(RADIUS as f32),
                material: Arc::new(self.get_material()),
            },
            "brick" => Object3D::brick(
                Vec3D::new(
                    self.input_elements["pos_x"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.x as f32),
                    self.input_elements["pos_y"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.y as f32),
                    self.input_elements["pos_z"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(POS.z as f32),
                ),
                Vec3D::new(
                    self.input_elements["size_x"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(SIZE as f32),
                    self.input_elements["size_y"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(SIZE as f32),
                    self.input_elements["size_z"]
                        .value()
                        .parse::<f32>()
                        .unwrap_or(SIZE as f32),
                ),
                Arc::new(self.get_material()),
            ),
            _ => unreachable!(),
        }
    }

    fn set_object(&self, object: &Object3D) {
        self.input_elements["pos_x"].set_value_as_number(POS.x);
        self.input_elements["pos_y"].set_value_as_number(POS.y);
        self.input_elements["pos_z"].set_value_as_number(POS.z);

        self.input_elements["size_x"].set_value_as_number(SIZE);
        self.input_elements["size_y"].set_value_as_number(SIZE);
        self.input_elements["size_z"].set_value_as_number(SIZE);

        self.input_elements["object_radius"].set_value_as_number(RADIUS);

        match object {
            Object3D::Sphere {
                pos,
                radius,
                material,
            } => {
                self.select_elements["object_type_select"].set_value("sphere");

                self.input_elements["pos_x"].set_value_as_number(pos.x.into());
                self.input_elements["pos_y"].set_value_as_number(pos.y.into());
                self.input_elements["pos_z"].set_value_as_number(pos.z.into());

                self.input_elements["object_radius"].set_value_as_number(*radius as f64);

                self.set_material(&*material)
            }
            Object3D::Brick {
                pos,
                corner,
                material,
                ..
            } => {
                self.select_elements["object_type_select"].set_value("brick");

                self.input_elements["pos_x"].set_value_as_number(pos.x.into());
                self.input_elements["pos_y"].set_value_as_number(pos.y.into());
                self.input_elements["pos_z"].set_value_as_number(pos.z.into());

                self.input_elements["size_x"].set_value_as_number((corner.x - pos.x).into());
                self.input_elements["size_y"].set_value_as_number((corner.y - pos.y).into());
                self.input_elements["size_z"].set_value_as_number((corner.z - pos.z).into());

                self.set_material(&*material)
            }
            Object3D::XYRect { .. } | Object3D::XZRect { .. } | Object3D::YZRect { .. } => {
                unimplemented!()
            }
        }
    }

    fn set_texture(&self, texture: &Texture) {
        match texture {
            Texture::SolidColor { color } => {
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["texture_select"].set_value("solid_color");

                self.subsection_elements["line_texture_color"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.input_elements["texture_color"].set_value(&color.to_string());
            }
            Texture::Checkered {
                odd_color,
                even_color,
                check_size,
            } => {
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("height", "28%")
                    .unwrap();

                self.select_elements["texture_select"].set_value("checkered");

                self.subsection_elements["line_texture_color_odd"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.input_elements["texture_color_odd"].set_value(&odd_color.to_string());

                self.subsection_elements["line_texture_color_even"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.input_elements["texture_color_even"].set_value(&even_color.to_string());

                self.subsection_elements["line_texture_check_size"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.input_elements["texture_check_size"].set_value_as_number(*check_size as f64);
            }
            Texture::Image { data } => {
                self.subsection_elements["texture_multi_box"]
                    .style()
                    .set_property("height", "16%")
                    .unwrap();

                self.select_elements["texture_select"].set_value("image");

                self.subsection_elements["line_texture_image"]
                    .style()
                    .set_property("display", "flex")
                    .unwrap();
                self.select_elements["texture_image_select"]
                    .set_value(&data.data.1.to_string().to_lowercase())
            }
        }
    }

    fn get_texture(&self) -> Texture {
        match self.select_elements["texture_select"].value().as_str() {
            "solid_color" => Texture::SolidColor {
                color: Color::from_hex_str(self.input_elements["texture_color"].value()),
            },
            "checkered" => Texture::Checkered {
                odd_color: Color::from_hex_str(self.input_elements["texture_color_odd"].value()),
                even_color: Color::from_hex_str(self.input_elements["texture_color_even"].value()),
                check_size: self.input_elements["texture_check_size"]
                    .value()
                    .parse::<f32>()
                    .unwrap_or(CHECK_SIZE as f32),
            },
            "image" => match self.select_elements["texture_image_select"]
                .value()
                .to_lowercase()
                .as_str()
            {
                "brick" => Texture::Image {
                    data: get_const_image(ImageID::Brick),
                },
                "earth" => Texture::Image {
                    data: get_const_image(ImageID::Earth),
                },
                "jupiter" => Texture::Image {
                    data: get_const_image(ImageID::Jupiter),
                },
                "mars" => Texture::Image {
                    data: get_const_image(ImageID::Mars),
                },
                "sun" => Texture::Image {
                    data: get_const_image(ImageID::Sun),
                },
                "rust" => Texture::Image {
                    data: get_const_image(ImageID::Rust),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn set_material(&self, material: &Material) {
        match material {
            Material::Lambertian { texture } => {
                self.set_texture(texture);
                self.update_sub_visibility(
                    self.select_elements["object_type_select"].value(),
                    String::from("lambertian"),
                );
            }
            Material::Metal { texture, fuzz } => {
                self.set_texture(texture);
                self.input_elements["material_fuzz"].set_value_as_number(*fuzz as f64);
                self.update_sub_visibility(
                    self.select_elements["object_type_select"].value(),
                    String::from("metal"),
                );
            }
            Material::Dielectric { refraction_index } => {
                self.input_elements["material_refraction_index"]
                    .set_value_as_number(*refraction_index as f64);
                self.update_sub_visibility(
                    self.select_elements["object_type_select"].value(),
                    String::from("dielectric"),
                );
            }
            Material::DiffuseLight { texture } => {
                self.set_texture(texture);
                self.update_sub_visibility(
                    self.select_elements["object_type_select"].value(),
                    String::from("diffuse_light"),
                );
            }
        }
    }

    fn get_material(&self) -> Material {
        match self.select_elements["material_select"].value().as_str() {
            "lambertian" => Material::Lambertian {
                texture: self.get_texture(),
            },
            "metal" => Material::Metal {
                texture: self.get_texture(),
                fuzz: self.input_elements["material_fuzz"].value_as_number() as f32,
            },
            "dielectric" => Material::Dielectric {
                refraction_index: self.input_elements["material_refraction_index"].value_as_number()
                    as f32,
            },
            "diffuse_light" => Material::DiffuseLight {
                texture: self.get_texture(),
            },
            _ => unreachable!(),
        }
    }
}

impl Default for EditObject {
    fn default() -> Self {
        let panel_element = get_element_by_id("object_panel");
        panel_element
            .style()
            .set_property("display", "flex")
            .unwrap();

        let input_elements = HashMap::from([
            (
                "pos_x".to_string(),
                get_element_by_id("pos_x")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "pos_y".to_string(),
                get_element_by_id("pos_y")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "pos_z".to_string(),
                get_element_by_id("pos_z")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "size_x".to_string(),
                get_element_by_id("size_x")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "size_y".to_string(),
                get_element_by_id("size_y")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "size_z".to_string(),
                get_element_by_id("size_z")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "object_radius".to_string(),
                get_element_by_id("object_radius")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "material_fuzz".to_string(),
                get_element_by_id("material_fuzz")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "texture_color".to_string(),
                get_element_by_id("texture_color")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "material_refraction_index".to_string(),
                get_element_by_id("material_refraction_index")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "texture_color".to_string(),
                get_element_by_id("texture_color")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "texture_color_odd".to_string(),
                get_element_by_id("texture_color_odd")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "texture_color_even".to_string(),
                get_element_by_id("texture_color_even")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "texture_check_size".to_string(),
                get_element_by_id("texture_check_size")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
        ]);

        let subsection_elements = HashMap::from([
            (
                "line_object_radius".to_string(),
                get_element_by_id("line_object_radius"),
            ),
            (
                "line_object_size".to_string(),
                get_element_by_id("line_object_size"),
            ),
            (
                "line_material_fuzz".to_string(),
                get_element_by_id("line_material_fuzz"),
            ),
            (
                "line_material_refraction_index".to_string(),
                get_element_by_id("line_material_refraction_index"),
            ),
            (
                "line_texture_color".to_string(),
                get_element_by_id("line_texture_color"),
            ),
            (
                "line_texture_color_odd".to_string(),
                get_element_by_id("line_texture_color_odd"),
            ),
            (
                "line_texture_color_even".to_string(),
                get_element_by_id("line_texture_color_even"),
            ),
            (
                "line_texture_check_size".to_string(),
                get_element_by_id("line_texture_check_size"),
            ),
            (
                "line_texture_image".to_string(),
                get_element_by_id("line_texture_image"),
            ),
            (
                "object_multi_box".to_string(),
                get_element_by_id("object_multi_box"),
            ),
            (
                "material_multi_box".to_string(),
                get_element_by_id("material_multi_box"),
            ),
            (
                "texture_multi_box".to_string(),
                get_element_by_id("texture_multi_box"),
            ),
            (
                "object_multi_box".to_string(),
                get_element_by_id("object_multi_box"),
            ),
        ]);

        let select_elements = HashMap::from([
            (
                "object_type_select".to_string(),
                get_element_by_id("object_type_select")
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap(),
            ),
            (
                "material_select".to_string(),
                get_element_by_id("material_select")
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap(),
            ),
            (
                "texture_select".to_string(),
                get_element_by_id("texture_select")
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap(),
            ),
            (
                "texture_image_select".to_string(),
                get_element_by_id("texture_image_select")
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap(),
            ),
        ]);

        let out = EditObject {
            panel_element,
            subsection_elements,
            select_elements,
            input_elements,
        };
        out.hide_sub_lines();

        out
    }
}

impl From<Object3D> for EditObject {
    fn from(item: Object3D) -> EditObject {
        let edit_object = EditObject::default();

        edit_object.set_object(&item);

        edit_object
    }
}

impl From<EditObject> for Object3D {
    fn from(item: EditObject) -> Object3D {
        item.get_object()
    }
}
