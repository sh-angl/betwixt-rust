use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use wasm_bindgen::JsValue;
use nalgebra::{Matrix4,Perspective3};


pub fn init_gfx(document: &Document) -> Result<WebGlRenderingContext, JsValue>{
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);



    gl.clear_color(0.0, 0.5, 0.0, 1.0);
    gl.clear_depth(1.);



    render(&gl);

    Ok(gl)

}

pub fn render(gl: &GL){
    link_program( &gl, &"", &"");
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT );

}

fn link_program(gl : &GL, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String>{
    let program = gl.create_program().ok_or_else(|| String::from("Error creating program")).unwrap();

    // creating vertex shader
    let vert_shader = GL::create_shader(
        &gl,
        GL::VERTEX_SHADER
    ).unwrap();
    GL::shader_source(gl, &vert_shader, vert_src);
    GL::compile_shader(gl, &vert_shader);
    
    // creating frag shader
    let frag_shader = GL::create_shader(
        &gl,
        GL::FRAGMENT_SHADER
    ).unwrap();
    GL::shader_source(gl, &frag_shader, vert_src);
    GL::compile_shader(gl, &frag_shader);

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

