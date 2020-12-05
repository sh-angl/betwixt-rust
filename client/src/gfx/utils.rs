use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsValue;
use nalgebra::{Matrix4,Perspective3};
use wasm_bindgen_futures as wasm_futures;
use futures;

async fn imgFutureWrapper(){

}

fn load_image(img_elem: &HtmlImageElement) /*-> Result<JsValue, JsValue>*/{
    let closure_await = wasm_bindgen::closure::Closure::wrap(Box::new(||{} as Box<dyn FnMut()>));
    let image_future = wasm_futures::JsFuture::from(img_elem.decode()/*.then(&closure_await)*/);
    let thing = wasm_futures::spawn_local(image_future);
    // Ok()
}

pub fn init_gfx(document: &Document) -> Result<WebGlRenderingContext, JsValue>{
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);



    gl.clear_color(0.0, 0.5, 0.0, 1.0);
    gl.clear_depth(1.);

    render(&gl);
    let other_gl = gl.clone();

    let image_stuff = document.get_element_by_id("image").unwrap().dyn_into::<web_sys::HtmlImageElement>()?;

    load_image(&image_stuff); //this should block until it isloaded afaik

    // wasm_futures::spawn_local(image_future);

    let thing = super::programs::image::Image::new(&other_gl, image_stuff);

    // thing.render(&gl);

    Ok(gl)

}

pub fn render(gl: &GL){
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );

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
