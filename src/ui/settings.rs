use crate::misc::get_element_by_id;
use crate::vector::Vec2D;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement};

// --------------------------------------------------

const DEFAULT_SCENE: u32 = 0;

const SAMPLES_PER_PIXEL_FULL_DEFAULT: f64 = 500.0;
const SAMPLES_PER_PIXEL_FULL_MIN_MAX: (f64, f64) = (50.0, 10000.0);

const SAMPLES_PER_PIXEL_PREVIEW_DEFAULT: f64 = 20.0;
const SAMPLES_PER_PIXEL_PREVIEW_MIN_MAX: (f64, f64) = (5.0, 100.0);

const RAY_BOUNCE_PER_RECURSION_LIMIT_DEFAULT: f64 = 50.0;
const RAY_BOUNCE_PER_RECURSION_LIMIT_MIN_MAX: (f64, f64) = (5.0, 500.0);

const VFOV_DEFAULT: f64 = 90.0;

const AUTO_RESIZE_DEFAULT: bool = true;
const RESOLUTION_X_DEFAULT: f64 = 1920.0;
const RESOLUTION_Y_DEFAULT: f64 = 1080.0;

// --------------------------------------------------

#[derive(Clone, Debug)]
pub struct Settings {
    pub show: bool,
    pub panel_element: HtmlElement,
    pub input_elements: HashMap<String, HtmlInputElement>,
    pub button_elements: HashMap<String, HtmlButtonElement>,
    pub file_input_element: HtmlInputElement,
    pub labels: HashMap<String, HtmlElement>,
}

impl Settings {
    pub fn set_scene_label(&self, val: u32) {
        self.labels["scene_label"].set_text_content(Some(format!("Scene {val:}").as_str()))
    }

    pub fn update(&self) {
        for elem in &self.input_elements {
            match elem.0.as_str() {
                "samples_per_pixel_full" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(
                        SAMPLES_PER_PIXEL_FULL_MIN_MAX.0,
                        SAMPLES_PER_PIXEL_FULL_MIN_MAX.1,
                    )),
                    Err(_) => elem.1.set_value_as_number(SAMPLES_PER_PIXEL_FULL_DEFAULT),
                },
                "samples_per_pixel_preview" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(
                        SAMPLES_PER_PIXEL_PREVIEW_MIN_MAX.0,
                        SAMPLES_PER_PIXEL_PREVIEW_MIN_MAX.1,
                    )),
                    Err(_) => elem
                        .1
                        .set_value_as_number(SAMPLES_PER_PIXEL_PREVIEW_DEFAULT),
                },
                "ray_bounce_recursion_limit" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(
                        RAY_BOUNCE_PER_RECURSION_LIMIT_MIN_MAX.0,
                        RAY_BOUNCE_PER_RECURSION_LIMIT_MIN_MAX.1,
                    )),
                    Err(_) => elem
                        .1
                        .set_value_as_number(RAY_BOUNCE_PER_RECURSION_LIMIT_DEFAULT),
                },
                "vfov" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(1.0, 180.0)),
                    Err(_) => elem.1.set_value_as_number(VFOV_DEFAULT),
                },
                "auto_resize" => {
                    if elem.1.checked() {
                        self.input_elements["resolution_x"].set_disabled(true);
                        self.input_elements["resolution_y"].set_disabled(true);
                    } else {
                        self.input_elements["resolution_x"].set_disabled(false);
                        self.input_elements["resolution_y"].set_disabled(false);
                    }
                }
                "resolution_x" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(20.0, 100000.0)),
                    Err(_) => elem.1.set_value_as_number(RESOLUTION_X_DEFAULT),
                },
                "resolution_y" => match elem.1.value().parse::<f64>() {
                    Ok(v) => elem.1.set_value_as_number(v.clamp(20.0, 100000.0)),
                    Err(_) => elem.1.set_value_as_number(RESOLUTION_Y_DEFAULT),
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn visibility(&self) -> &bool {
        &self.show
    }

    pub fn set_visibility(&mut self, show: bool) {
        self.show = show;
        self.panel_element
            .style()
            .set_property("display", if self.show { "flex" } else { "none" })
            .unwrap();
    }

    pub fn samples_per_pixel_full(&self) -> u32 {
        match self.input_elements["samples_per_pixel_full"]
            .value()
            .parse::<u32>()
        {
            Ok(v) => v,
            Err(_) => SAMPLES_PER_PIXEL_FULL_DEFAULT as u32,
        }
    }

    pub fn samples_per_pixel_preview(&self) -> u32 {
        match self.input_elements["samples_per_pixel_preview"]
            .value()
            .parse::<u32>()
        {
            Ok(v) => v,
            Err(_) => SAMPLES_PER_PIXEL_PREVIEW_DEFAULT as u32,
        }
    }

    pub fn ray_bounce_recursion_limit(&self) -> i32 {
        match self.input_elements["ray_bounce_recursion_limit"]
            .value()
            .parse::<i32>()
        {
            Ok(v) => v,
            Err(_) => RAY_BOUNCE_PER_RECURSION_LIMIT_DEFAULT as i32,
        }
    }

    pub fn vfov(&self) -> f32 {
        match self.input_elements["vfov"].value().parse::<f32>() {
            Ok(v) => v,
            Err(_) => RAY_BOUNCE_PER_RECURSION_LIMIT_DEFAULT as f32,
        }
    }

    pub fn auto_resize(&self) -> bool {
        self.input_elements["auto_resize"].checked()
    }

    pub fn resolution(&self) -> Vec2D<u32> {
        match self.input_elements["resolution_x"].value().parse::<u32>() {
            Ok(x) => match self.input_elements["resolution_y"].value().parse::<u32>() {
                Ok(y) => Vec2D::new(x as u32, y as u32),
                Err(_) => Vec2D::new(RESOLUTION_X_DEFAULT as u32, RESOLUTION_Y_DEFAULT as u32),
            },
            Err(_) => Vec2D::new(RESOLUTION_X_DEFAULT as u32, RESOLUTION_Y_DEFAULT as u32),
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.panel_element
            .style()
            .set_property("display", "none")
            .unwrap();
    }
}

impl Default for Settings {
    fn default() -> Self {
        let setting_panel_element = get_element_by_id("settings_panel");
        setting_panel_element
            .style()
            .set_property("display", "flex")
            .unwrap();

        let input_elements = HashMap::from([
            (
                "samples_per_pixel_full".to_string(),
                get_element_by_id("samples_per_pixel_full")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "samples_per_pixel_preview".to_string(),
                get_element_by_id("samples_per_pixel_preview")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "ray_bounce_recursion_limit".to_string(),
                get_element_by_id("ray_bounce_recursion_limit")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "vfov".to_string(),
                get_element_by_id("vfov")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "auto_resize".to_string(),
                get_element_by_id("auto_resize")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "resolution_x".to_string(),
                get_element_by_id("resolution_x")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
            (
                "resolution_y".to_string(),
                get_element_by_id("resolution_y")
                    .dyn_into::<HtmlInputElement>()
                    .unwrap(),
            ),
        ]);
        let labels = HashMap::from([("scene_label".to_string(), get_element_by_id("scene_label"))]);

        let button_elements = HashMap::from([
            (
                "previous_scene".to_string(),
                get_element_by_id("previous_scene")
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap(),
            ),
            (
                "next_scene".to_string(),
                get_element_by_id("next_scene")
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap(),
            ),
            (
                "download_scene".to_string(),
                get_element_by_id("download_scene")
                    .dyn_into::<HtmlButtonElement>()
                    .unwrap(),
            ),
        ]);

        let file_input_element = get_element_by_id("scene_from_file")
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        // Initialize
        for elem in &input_elements {
            match elem.0.as_str() {
                "samples_per_pixel_full" => {
                    elem.1.set_value_as_number(SAMPLES_PER_PIXEL_FULL_DEFAULT)
                }
                "samples_per_pixel_preview" => elem
                    .1
                    .set_value_as_number(SAMPLES_PER_PIXEL_PREVIEW_DEFAULT),
                "ray_bounce_recursion_limit" => elem
                    .1
                    .set_value_as_number(RAY_BOUNCE_PER_RECURSION_LIMIT_DEFAULT),
                "vfov" => elem.1.set_value_as_number(VFOV_DEFAULT),
                "auto_resize" => elem.1.set_checked(AUTO_RESIZE_DEFAULT),
                "resolution_x" => {
                    elem.1.set_value_as_number(RESOLUTION_X_DEFAULT);
                    elem.1.set_disabled(true);
                }
                "resolution_y" => {
                    elem.1.set_value_as_number(RESOLUTION_Y_DEFAULT);
                    elem.1.set_disabled(true);
                }
                _ => {
                    unreachable!()
                }
            }
        }
        for elem in &labels {
            match elem.0.as_str() {
                "scene_label" => {
                    elem.1
                        .set_text_content(Some(format!("Scene {DEFAULT_SCENE:}").as_str()));
                }
                _ => {}
            }
        }

        Settings {
            show: true,
            panel_element: setting_panel_element,
            input_elements,
            button_elements,
            file_input_element,
            labels,
        }
    }
}
