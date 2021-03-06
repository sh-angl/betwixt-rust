use std::{borrow::Borrow, sync::Arc};
use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsValue;
use nalgebra::{Matrix4,Perspective3};
use wasm_bindgen_futures as wasm_futures;
use futures;

use crate::scene::Scene;

// fn load_image(img_elem: &HtmlImageElement) /*-> Result<JsValue, JsValue>*/{
//     let closure_await = wasm_bindgen::closure::Closure::wrap(Box::new(||{} as Box<dyn FnMut()>));
//     let image_future = wasm_futures::JsFuture::from(img_elem.decode()/*.then(&closure_await)*/);
//     let thing = wasm_futures::spawn_local(image_future);
//     // Ok()
// }

#[allow(unused_unsafe)]
pub fn init_gfx(document: &Document, scene: &mut Scene) -> Result<WebGlRenderingContext, JsValue>{
    console_log!("initing gfx");
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    console_log!("got canvas el");
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    console_log!("got el again");
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;
    console_log!("got gl context");
    
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

    //render(&gl);
    // console_log!("rendered");
    // let other_gl = gl.clone();
    // console_log!("cloned gl");

    let image_stuff = document.get_element_by_id("image").unwrap().dyn_into::<web_sys::HtmlImageElement>()?;
    // console_log!("got image {}", image_stuff.src());

    // load_image(&image_stuff); //this should block until it isloaded afaik

    // wasm_futures::spawn_local(image_future);

    let thing = super::programs::sprite::Sprite::new(&gl, image_stuff);
    scene.sprites.push(Arc::new(thing));
    
    console_log!("new image");

    // thing.render(&gl);

    Ok(gl)

}

pub fn link_program(gl : &GL, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String>{
    let program = gl.create_program().ok_or_else(|| String::from("Error creating program")).unwrap();

    // creating vertex shader
    let vert_shader = GL::create_shader(
        &gl,
        GL::VERTEX_SHADER
    ).unwrap();
    GL::shader_source(gl, &vert_shader, vert_src);
    GL::compile_shader(gl, &vert_shader);

    // check if vert compiled
    if !gl.get_shader_parameter(&vert_shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false){
        let other = gl.get_shader_info_log(&vert_shader).unwrap();
        console_log!("vert: {}", other);
    }

    
    // creating frag shader
    let frag_shader = GL::create_shader(
        &gl,
        GL::FRAGMENT_SHADER
    ).unwrap();
    GL::shader_source(gl, &frag_shader, frag_src);
    GL::compile_shader(gl, &frag_shader);

    // check if frag is compiled
    if !gl.get_shader_parameter(&frag_shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false){
        let other = gl.get_shader_info_log(&frag_shader).unwrap();
        console_log!("frag: {}", other);
    }



    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

