use std::ptr::null;

use web_sys::{EventListener, MouseEvent, Window, FocusEvent, WheelEvent};
use web_sys::KeyboardEvent;
use web_sys::{Document};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{state::State,STATE, window, document};
// #[macro_use]
// mod dev_utils;

struct InputVector{

}

fn keydown(event: KeyboardEvent) {
  console_log!("down: {}", event.key_code());
  match event.key_code(){
    65 => console_log!("left"),
    83 => console_log!("down"),
    68 => console_log!("right"),
    87 => console_log!("up"),
    _ => ()
  }
}

fn keyup(event: KeyboardEvent) {
}

fn mousedown(event: MouseEvent) {
}

fn mouseup(event: MouseEvent) {
}

fn mousemove(event: MouseEvent) {
}

fn mousescroll(event: WheelEvent) {
  // console_log!("ydelta: {}", event.delta_y());
  let mut state = STATE.lock().unwrap();
  let mut new_zoom = state.camera.zoom() - (event.delta_y() as f32 / 10.);
  state.camera.set_zoom(new_zoom);
  //console_log!("zoom: {}", new_zoom);
}

fn resize() { // FIXME: window resize debouncing
  let win = window();
  let mut state = STATE.lock().unwrap();
  let width = win.inner_width().unwrap().as_f64().unwrap();
  let height = win.inner_height().unwrap().as_f64().unwrap();
  state.camera.set_screen_size(width, height);
  state.dirty_screen = true;

  //console_log!("screen size: {}x{}", width, height);

  
  let canvas = document().get_element_by_id("rustCanvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  canvas.set_width(width as u32);
  canvas.set_height(height as u32);
}

pub fn init_controls(document: &Document, window: &Window){
    let handle = Closure::wrap(Box::new(keydown) as Box<dyn FnMut(KeyboardEvent)>);
    document.add_event_listener_with_callback("keydown", handle.as_ref().unchecked_ref());
    handle.forget();
    let handle = Closure::wrap(Box::new(keyup) as Box<dyn FnMut(KeyboardEvent)>);
    document.add_event_listener_with_callback("keyup", handle.as_ref().unchecked_ref());
    handle.forget();

    let handle = Closure::wrap(Box::new(mousemove) as Box<dyn FnMut(MouseEvent)>);
    document.add_event_listener_with_callback("mousemove", handle.as_ref().unchecked_ref());
    handle.forget();
    let handle = Closure::wrap(Box::new(mousedown) as Box<dyn FnMut(MouseEvent)>);
    document.add_event_listener_with_callback("mousedown", handle.as_ref().unchecked_ref());
    handle.forget();
    let handle = Closure::wrap(Box::new(mouseup) as Box<dyn FnMut(MouseEvent)>);
    document.add_event_listener_with_callback("mouseup", handle.as_ref().unchecked_ref());
    handle.forget();
    let handle = Closure::wrap(Box::new(mousescroll) as Box<dyn FnMut(WheelEvent)>);
    document.add_event_listener_with_callback("wheel", handle.as_ref().unchecked_ref());
    handle.forget();

    let handle = Closure::wrap(Box::new(resize) as Box<dyn FnMut()>);
    window.add_event_listener_with_callback("resize", handle.as_ref().unchecked_ref());
    resize();
    handle.forget();
    
}