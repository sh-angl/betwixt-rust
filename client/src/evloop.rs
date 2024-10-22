use std::{borrow::Borrow, cell::RefCell, sync::Arc};

use nalgebra::Vector3;
use web_sys::EventListener;
use web_sys::KeyboardEvent;
use web_sys::{Document};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext as GL;

use crate::{body, scene::Scene, state::State, window};
use crate::gfx::programs::sprite::Sprite;
use std::rc::Rc;

/// This function is called at a semi-fixed rate (<= fixedStep) for physics, etc. Avoid any drawing related functionality in here.
pub fn tick(state: &State, scene: &Scene) {
}

/// This function is called every frame.
pub fn draw(gl: &GL, state: &mut State, scene: &Scene) {
  if state.dirty_screen {
    state.dirty_screen = false;
    let window = window();
    gl.viewport(0,0, window.inner_width().unwrap().as_f64().unwrap() as i32, window.inner_height().unwrap().as_f64().unwrap() as i32);
  }

  gl.clear_color(0.0, 0.5, 0.0, 1.0);
  gl.clear_depth(1.);
  gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );

  scene.sprites.iter().clone().for_each(|spr: &Arc<Sprite>| {
    spr.render(gl, &state.camera, &Vector3::new(0., state.time.sin() as f32, 0.));
  });

  // body().set_text_content(Some(&format!("dt: {}", dt)));
}