use std::sync::Arc;

use crate::ui::Settings;
use crate::vector::Vec2D;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// --------------------------------------------------

const OUTPUT_CANVAS_NAME: &str = "render_canvas";

// --------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum RenderState {
    #[default]
    StandByMode,
    PreviewRender,
    FullRender,
    RenderingFreeze,
    FinishedFullRender,
}

// --------------------------------------------------

#[wasm_bindgen(module = "/helper_methods.js")]
extern "C" {
    pub fn draw(canvas_name: String, width: u32, data: Vec<u8>);
    pub fn cpu_cores() -> u32;
    pub fn download_blob(blob: web_sys::Blob, filename: String);
    pub fn blob_from_str(data: String) -> web_sys::Blob;
    pub fn event_to_file(event: web_sys::Event) -> web_sys::Blob;
}

// --------------------------------------------------

pub fn get_element_by_id(id: &str) -> web_sys::HtmlElement {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Element '{id:?}' does not exist"))
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap_or_else(|_| panic!("Could not dyn element '{id:?}' into HtmlElement"))
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn canvas_html() -> web_sys::HtmlCanvasElement {
    get_element_by_id(OUTPUT_CANVAS_NAME)
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

// --------------------------------------------------

pub fn download_struct(object: impl Serialize, filename: String) {
    let filename_arc = Arc::new(filename);

    download_blob(
        blob_from_str(serde_yaml::to_string(&object).unwrap()),
        format!("{:?}.yaml", *filename_arc),
    );
}

pub fn download_canvas_content(filename: String) {
    let filename_arc = Arc::new(filename);

    let closure = Closure::wrap(Box::new(move |blob: web_sys::Blob| {
        let filename_arc = Arc::clone(&filename_arc);
        download_blob(blob, (*filename_arc).clone())
    }) as Box<dyn FnMut(_)>);

    canvas_html()
        .to_blob(closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

pub fn flip_vec_u8_to_canvas(bitmap: (Vec<u8>, Vec2D<u32>)) {
    let bitmap_data = bitmap.0;
    let bitmap_size = bitmap.1;

    draw(OUTPUT_CANVAS_NAME.to_string(), bitmap_size.x, bitmap_data)
}

// --------------------------------------------------

pub fn update_window_size(settings: &Settings, view_size: &mut Vec2D<u32>) {
    let window = window();
    let canvas_html = canvas_html();

    *view_size = if settings.auto_resize() {
        Vec2D::new(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        )
    } else {
        settings.resolution()
    };

    canvas_html.set_width(view_size.x);
    canvas_html.set_height(view_size.y);
}
