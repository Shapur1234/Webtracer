use std::sync::{Arc, Mutex};

use crate::graphics::{self, CameraInput, Object3D, SceneList};
use crate::misc::{
    canvas_html, document, flip_vec_u8_to_canvas, update_window_size, window, RenderState,
};
use crate::ui::{EditObject, Settings};
use crate::vector::Vec2D;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn register_ui_events(
    camera_input: Arc<Mutex<CameraInput>>,
    pointerlock_state: Arc<Mutex<bool>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    register_draw_timer(
        Arc::clone(&render_state),
        Arc::clone(&edit_object),
        Arc::clone(&settings),
    );

    register_mouse_click(Arc::clone(&pointerlock_state), Arc::clone(&render_state));
    register_pointer_lock_change(
        Arc::clone(&pointerlock_state),
        Arc::clone(&render_state),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
    );
    register_pointerlock_error(Arc::clone(&pointerlock_state));

    register_window_resize(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&view_size),
    );

    register_keyboard_input(
        Arc::clone(&camera_input),
        Arc::clone(&pointerlock_state),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
        Arc::clone(&view_size),
    );
    register_mouse_move(
        Arc::clone(&camera_input),
        Arc::clone(&pointerlock_state),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&view_size),
    );

    register_object_input(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
        Arc::clone(&view_size),
    );
    register_settings_input_fields(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&view_size),
    );
    register_settings_prev_scene(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
        Arc::clone(&view_size),
    );
    register_settings_next_scene(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&edit_object),
        Arc::clone(&view_size),
    );
    register_settings_download_scene(Arc::clone(&scenes), Arc::clone(&settings));
    register_upload_scene_from_file(
        Arc::clone(&camera_input),
        Arc::clone(&render_state),
        Arc::clone(&scenes),
        Arc::clone(&settings),
        Arc::clone(&view_size),
    );
}

// --------------------------------------------------

pub fn render_frame(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    // Todo: Fix closure to not reload every time
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut camera_input_1 = camera_input.lock().unwrap();
        let mut render_state_1 = render_state.lock().unwrap();
        let mut scenes_1 = scenes.lock().unwrap();
        let settings_1 = settings.lock().unwrap();
        let view_size_1 = view_size.lock().unwrap();

        scenes_1.change_size(*view_size_1);

        let scene = scenes_1.current();
        scene.camera.handle_input(&camera_input_1);
        camera_input_1.reset();

        match *render_state_1 {
            RenderState::FullRender => {
                document().exit_pointer_lock();

                scene.render_full(
                    settings_1.samples_per_pixel_full(),
                    settings_1.ray_bounce_recursion_limit(),
                    true,
                );
                *render_state_1 = RenderState::RenderingFreeze
            }
            RenderState::PreviewRender => {
                document().exit_pointer_lock();

                scene.render_full(
                    settings_1.samples_per_pixel_preview(),
                    settings_1.ray_bounce_recursion_limit(),
                    true,
                    // false,
                );
                *render_state_1 = RenderState::RenderingFreeze
            }
            RenderState::StandByMode => {
                flip_vec_u8_to_canvas(scene.render_simple().to_vec_u8(true))
            }
            _ => *render_state_1 = RenderState::default(),
        };
    });

    window()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

fn register_draw_timer(
    render_state: Arc<Mutex<RenderState>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    settings: Arc<Mutex<Settings>>,
) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut render = crate::graphics::RENDER.lock().unwrap();
        if render.0 {
            let mut render_state_1 = render_state.lock().unwrap();
            let mut settings_1 = settings.lock().unwrap();
            let edit_object_1 = edit_object.lock().unwrap();

            if let Some(panel) = &*edit_object_1 {
                panel.hide()
            }
            (*settings_1).set_visibility(false);

            flip_vec_u8_to_canvas(render.1.to_vec_u8(true));

            render.0 = false;
            *render_state_1 = RenderState::FinishedFullRender;

            web_sys::console::time_end_with_label("Render start");
        }
    });

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            500,
        )
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

fn register_window_resize(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let settings_1 = settings.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();
        let mut view_size_1 = view_size.lock().unwrap();

        match *render_state_1 {
            RenderState::FinishedFullRender | RenderState::RenderingFreeze => {}
            _ => {
                update_window_size(&*settings_1, &mut *view_size_1);

                render_frame(
                    Arc::clone(&camera_input),
                    Arc::clone(&render_state),
                    Arc::clone(&scenes),
                    Arc::clone(&settings),
                    Arc::clone(&view_size),
                );
            }
        }
    });

    window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

fn register_mouse_click(
    pointerlock_state: Arc<Mutex<bool>>,
    render_state: Arc<Mutex<RenderState>>,
) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let pointerlock_state_1 = pointerlock_state.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();

        if !*pointerlock_state_1
            && !matches!(
                *render_state_1,
                RenderState::RenderingFreeze | RenderState::FinishedFullRender
            )
        {
            canvas_html().request_pointer_lock();
        }
    });

    canvas_html()
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_pointer_lock_change(
    pointerlock_state: Arc<Mutex<bool>>,
    render_state: Arc<Mutex<RenderState>>,
    settings: Arc<Mutex<Settings>>,
    edit_panel: Arc<Mutex<Option<EditObject>>>,
) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut pointerlock_state_1 = pointerlock_state.lock().unwrap();
        let mut settings_1 = settings.lock().unwrap();
        let mut edit_panel_1 = edit_panel.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();

        if !*pointerlock_state_1 {
            match *render_state_1 {
                RenderState::RenderingFreeze | RenderState::FinishedFullRender => {}
                _ => {
                    *pointerlock_state_1 = true;

                    settings_1.set_visibility(false);
                    if let Some(panel) = &*edit_panel_1 {
                        panel.hide();
                        *edit_panel_1 = None;
                    }
                }
            }
        } else {
            *pointerlock_state_1 = false;
        }
    });

    document()
        .add_event_listener_with_callback("pointerlockchange", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_pointerlock_error(pointerlock_state: Arc<Mutex<bool>>) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut pointerlock_state_1 = pointerlock_state.lock().unwrap();

        *pointerlock_state_1 = false;

        document().exit_pointer_lock();
    });

    document()
        .add_event_listener_with_callback("pointerlockerror", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

fn register_keyboard_input(
    camera_input: Arc<Mutex<CameraInput>>,
    pointerlock_state: Arc<Mutex<bool>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut camera_input_1 = camera_input.lock().unwrap();
        let mut render_state_1 = render_state.lock().unwrap();
        let mut settings_1 = settings.lock().unwrap();
        let mut edit_object_1 = edit_object.lock().unwrap();
        let mut view_size_1 = view_size.lock().unwrap();
        let mut scenes_1 = scenes.lock().unwrap();
        let pointerlock_state_1 = pointerlock_state.lock().unwrap();

        let pressed_key = event.key_code();

        if !matches!(*render_state_1, RenderState::RenderingFreeze) {
            let camera_input_old = camera_input_1.clone();
            let render_state_old = render_state_1.clone();

            match *render_state_1 {
                RenderState::FinishedFullRender => {
                    if pressed_key == 89 {
                        *render_state_1 = RenderState::default();
                        update_window_size(&*settings_1, &mut *view_size_1);
                        render_frame(
                            Arc::clone(&camera_input),
                            Arc::clone(&render_state),
                            Arc::clone(&scenes),
                            Arc::clone(&settings),
                            Arc::clone(&view_size),
                        );
                    }
                }
                _ => {
                    if *pointerlock_state_1 {
                        *camera_input_1 = CameraInput {
                            forward: pressed_key == 38 || pressed_key == 87,
                            back: pressed_key == 40 || pressed_key == 83,
                            right: pressed_key == 39 || pressed_key == 68,
                            left: pressed_key == 37 || pressed_key == 65,
                            up: pressed_key == 75,
                            down: pressed_key == 74,
                            reset: pressed_key == 81,
                            mouse_move: (*camera_input_1).mouse_move,
                        };
                    }

                    if matches!(*render_state_1, RenderState::StandByMode) {
                        if pressed_key == 69 {
                            if let Some(panel) = &*edit_object_1 {
                                panel.hide();
                                *edit_object_1 = None;
                            } else {
                                *edit_object_1 = if let Some(object) =
                                    scenes_1.current().get_pointed_at_object()
                                {
                                    if matches!(*pointerlock_state_1, true) {
                                        document().exit_pointer_lock();
                                    }
                                    Some(EditObject::from((*object).clone()))
                                } else {
                                    None
                                }
                            }
                        }

                        if pressed_key == 88 {
                            if let Some(object_index) =
                                scenes_1.current().get_pointed_at_object_index()
                            {
                                scenes_1.current().object_list.objects.remove(object_index);
                            }
                        }

                        if pressed_key == 67 {
                            let current_scene = scenes_1.current_immut().clone();
                            let current_scene_mut = scenes_1.current();

                            current_scene_mut
                                .object_list
                                .objects
                                .push(Object3D::Sphere {
                                    pos: current_scene.camera.pos + current_scene.camera.rotation,
                                    radius: 0.4,
                                    material: Arc::new(graphics::Material::default()),
                                });
                            current_scene_mut.object_list = current_scene_mut
                                .object_list
                                .camera_sorted(&current_scene.camera);
                        }
                    }

                    {
                        match pressed_key {
                            82 => *render_state_1 = RenderState::PreviewRender,
                            70 => *render_state_1 = RenderState::FullRender,
                            _ => {}
                        }

                        if *camera_input_1 != camera_input_old
                            || *render_state_1 != render_state_old
                        {
                            update_window_size(&*settings_1, &mut *view_size_1);

                            render_frame(
                                Arc::clone(&camera_input),
                                Arc::clone(&render_state),
                                Arc::clone(&scenes),
                                Arc::clone(&settings),
                                Arc::clone(&view_size),
                            );
                        }
                    }
                }
            }

            if pressed_key == 85 {
                let download_name = format!("render_{:?}", thread_rng().gen::<u32>());
                log::info!("Downloaded as {download_name:?}");
                crate::misc::download_canvas_content(download_name);
            }
        }

        if pressed_key == 84 {
            let show = (*settings_1).show;
            settings_1.set_visibility(show ^ (pressed_key == 84));

            if *settings_1.visibility() {
                document().exit_pointer_lock();
            }
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_mouse_move(
    camera_input: Arc<Mutex<CameraInput>>,
    pointerlock_state: Arc<Mutex<bool>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let mut camera_input_1 = camera_input.lock().unwrap();
        let mut view_size_1 = view_size.lock().unwrap();
        let pointerlock_state_1 = pointerlock_state.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();
        let settings_1 = settings.lock().unwrap();

        if *pointerlock_state_1 && *render_state_1 == RenderState::StandByMode {
            (*camera_input_1).mouse_move = Some(Vec2D::new(
                event.movement_x() as f32,
                event.movement_y() as f32,
            ));
            update_window_size(&*settings_1, &mut *view_size_1);

            render_frame(
                Arc::clone(&camera_input),
                Arc::clone(&render_state),
                Arc::clone(&scenes),
                Arc::clone(&settings),
                Arc::clone(&view_size),
            );
        }
    }) as Box<dyn FnMut(_)>);

    canvas_html()
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

// --------------------------------------------------

fn register_object_input(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let settings_2 = Arc::clone(&settings);
    let closure = Closure::<dyn Fn()>::new(Box::new(move || {
        let mut scenes_1 = scenes.lock().unwrap();
        let mut edit_object_1 = edit_object.lock().unwrap();

        if let Some(panel) = &mut *edit_object_1 {
            if let Some(object) = scenes_1.current().get_pointed_at_object() {
                panel.update();
                *object = <EditObject as Into<Object3D>>::into(panel.clone());
                *panel = EditObject::from((*object).clone())
            } else {
                panel.hide();
            }

            if let Some(_) = scenes_1.current().get_pointed_at_object() {
            } else {
                panel.hide();
            }

            render_frame(
                Arc::clone(&camera_input),
                Arc::clone(&render_state),
                Arc::clone(&scenes),
                Arc::clone(&settings_2),
                Arc::clone(&view_size),
            );
        }
    }));

    {
        let edit_object_temp = EditObject::default();

        for elem in &edit_object_temp.input_elements {
            elem.1
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .unwrap();
        }
        for elem in &edit_object_temp.select_elements {
            elem.1
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .unwrap();
        }

        edit_object_temp.hide()
    }

    closure.forget();
}

fn register_settings_input_fields(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let settings_2 = Arc::clone(&settings);
    let closure = Closure::<dyn Fn()>::new(Box::new(move || {
        let mut scenes_1 = scenes.lock().unwrap();
        let mut view_size_1 = view_size.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();
        let settings_1 = settings_2.lock().unwrap();

        settings_1.update();

        match *render_state_1 {
            RenderState::FinishedFullRender => {}
            RenderState::RenderingFreeze => {}
            _ => {
                scenes_1.change_vfov(settings_1.vfov());
                update_window_size(&*settings_1, &mut *view_size_1);

                render_frame(
                    Arc::clone(&camera_input),
                    Arc::clone(&render_state),
                    Arc::clone(&scenes),
                    Arc::clone(&settings_2),
                    Arc::clone(&view_size),
                );
            }
        }
    }));

    for elem in &settings.lock().unwrap().input_elements {
        elem.1
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .unwrap();
    }

    closure.forget();
}

fn register_settings_prev_scene(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let settings_2 = Arc::clone(&settings);
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut scenes_1 = scenes.lock().unwrap();
        let mut edit_object_1 = edit_object.lock().unwrap();
        let settings_1 = settings_2.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();

        match *render_state_1 {
            RenderState::FinishedFullRender => {}
            RenderState::RenderingFreeze => {}
            _ => {
                (*scenes_1).prev();
                settings_1.set_scene_label((*scenes_1).current_scene_index());

                if let Some(edit) = &*edit_object_1 {
                    edit.hide();
                    *edit_object_1 = None;
                }

                render_frame(
                    Arc::clone(&camera_input),
                    Arc::clone(&render_state),
                    Arc::clone(&scenes),
                    Arc::clone(&settings_2),
                    Arc::clone(&view_size),
                );
            }
        }
    });

    settings.lock().unwrap().button_elements["previous_scene"]
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_settings_next_scene(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    edit_object: Arc<Mutex<Option<EditObject>>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let settings_2 = Arc::clone(&settings);
    let closure = Closure::<dyn Fn()>::new(move || {
        let mut scenes_1 = scenes.lock().unwrap();
        let mut edit_object_1 = edit_object.lock().unwrap();
        let settings_1 = settings_2.lock().unwrap();
        let render_state_1 = render_state.lock().unwrap();

        match *render_state_1 {
            RenderState::FinishedFullRender => {}
            RenderState::RenderingFreeze => {}
            _ => {
                (*scenes_1).next();
                settings_1.set_scene_label((*scenes_1).current_scene_index());

                if let Some(edit) = &*edit_object_1 {
                    edit.hide();
                    *edit_object_1 = None;
                }

                render_frame(
                    Arc::clone(&camera_input),
                    Arc::clone(&render_state),
                    Arc::clone(&scenes),
                    Arc::clone(&settings_2),
                    Arc::clone(&view_size),
                );
            }
        }
    });

    settings.lock().unwrap().button_elements["next_scene"]
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_settings_download_scene(scenes: Arc<Mutex<SceneList>>, settings: Arc<Mutex<Settings>>) {
    let closure = Closure::<dyn Fn()>::new(move || {
        let scenes_1 = scenes.lock().unwrap();

        crate::misc::download_struct(
            (*scenes_1).current_immut(),
            format!("scene_{:}", (*scenes_1).current_scene_index()),
        );
    });

    settings.lock().unwrap().button_elements["download_scene"]
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn register_upload_scene_from_file(
    camera_input: Arc<Mutex<CameraInput>>,
    render_state: Arc<Mutex<RenderState>>,
    scenes: Arc<Mutex<SceneList>>,
    settings: Arc<Mutex<Settings>>,
    view_size: Arc<Mutex<Vec2D<u32>>>,
) {
    let settings_2 = Arc::clone(&settings);

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let camera_input_1 = Arc::clone(&camera_input);
        let render_state_1 = Arc::clone(&render_state);
        let scenes_1 = Arc::clone(&scenes);
        let settings_1 = Arc::clone(&settings_2);
        let view_size_1 = Arc::clone(&view_size);

        let file_reader = Arc::new(web_sys::FileReader::new().unwrap());
        file_reader
            .read_as_text(&crate::misc::event_to_file(event))
            .unwrap();

        {
            let camera_input_1 = Arc::clone(&camera_input_1);
            let render_state_1 = Arc::clone(&render_state_1);
            let scenes_1 = Arc::clone(&scenes_1);
            let settings_1 = Arc::clone(&settings_1);
            let view_size_1 = Arc::clone(&view_size_1);

            if !matches!(*render_state.lock().unwrap(), RenderState::RenderingFreeze) {
                let file_reader_2 = Arc::clone(&file_reader);

                let file_onload_closure =
                    Closure::<dyn Fn()>::new(move || match file_reader_2.result() {
                        Ok(v) => {
                            if let Some(text) = v.as_string() {
                                match serde_yaml::from_str::<crate::graphics::Scene>(&text) {
                                    Ok(v) => {
                                        let mut scenes = scenes_1.lock().unwrap();
                                        *scenes.current() = v;
                                        scenes.camera_reset();

                                        render_frame(
                                            Arc::clone(&camera_input_1),
                                            Arc::clone(&render_state_1),
                                            Arc::clone(&scenes_1),
                                            Arc::clone(&settings_1),
                                            Arc::clone(&view_size_1),
                                        )
                                    }
                                    Err(e) => {
                                        log::error!("Failed parsing string as object: {e:?}")
                                    }
                                }
                            } else {
                                log::error!("Failed parsing read file as string")
                            }
                        }
                        Err(e) => {
                            log::error!("Failed reading text from file: {e:?}")
                        }
                    });

                file_reader.set_onload(Some(file_onload_closure.as_ref().unchecked_ref()));

                file_onload_closure.forget();
            }
        }
    }) as Box<dyn FnMut(_)>);

    settings
        .lock()
        .unwrap()
        .file_input_element
        .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}
