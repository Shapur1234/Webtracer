#![feature(const_fn_floating_point_arithmetic)]

mod draw;
mod event;
mod graphics;
mod image;
mod misc;
mod predefined_scenes;
mod ui;
mod vector;

use std::sync::{Arc, Mutex};

use graphics::{CameraInput, SceneList};
use misc::{update_window_size, RenderState};
use predefined_scenes::predefined_scenes;
use ui::Settings;
use vector::Vec2D;
use wasm_bindgen::prelude::*;

// TODO
// Fix jitter when exiting menus
// Add some scenes
// Fix brick ui

// --------------------------------------------------

#[wasm_bindgen(start)]
pub fn dummy_main() {}

#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init().expect("error initializing console_log");

    main();
}

fn main() {
    let camera_input = Arc::new(Mutex::new(CameraInput::default()));
    let pointerlock_state = Arc::new(Mutex::new(false));
    let render_state = Arc::new(Mutex::new(RenderState::StandByMode));
    let settings = Arc::new(Mutex::new(Settings::default()));
    let edit_object = Arc::new(Mutex::new(None));
    let view_size = Arc::new(Mutex::new(Vec2D::new(0u32, 0u32)));
    let scenes = Arc::new(Mutex::new(SceneList::new(
        predefined_scenes(),
        *view_size.lock().unwrap(),
        settings.lock().unwrap().vfov(),
    )));

    event::register_ui_events(
        Arc::clone(&camera_input),
        Arc::clone(&pointerlock_state),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
        Arc::clone(&view_size),
    );

    // --------------------------------------------------

    update_window_size(&*settings.lock().unwrap(), &mut *view_size.lock().unwrap());
    event::render_frame(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&view_size),
    );
}
