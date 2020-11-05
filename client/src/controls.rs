use web_sys::EventListener;
use web_sys::KeyboardEvent;
use web_sys::{Document};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// #[macro_use]
// mod dev_utils;

struct InputVector{

}

pub fn init_controls(document: &Document){
    let keydown_handler = Closure::wrap(Box::new(|event: KeyboardEvent|{
        console_log!("{}", event.key_code());
        match event.key_code(){
            65 => console_log!("left"),
            83 => console_log!("down"),
            68 => console_log!("right"),
            87 => console_log!("up"),
            _ => ()
        }

    }) as Box<dyn FnMut(KeyboardEvent)>);

    document.add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref());
    keydown_handler.forget();
}