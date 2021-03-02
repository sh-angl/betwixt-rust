use wasm_bindgen::{JsCast};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::Promise;
use wasm_bindgen_futures::*;
use wasm_bindgen::prelude::*;

use super::super::utils;
use super::super::shaders::{frag, vert};

// imports for stolen imageFuture code
use futures::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use std::cell::Cell;
use std::rc::Rc;

pub struct Image{
    program: WebGlProgram
}

impl Image{
    pub fn new(gl: &GL, img: HtmlImageElement) -> Self{
        console_log!("starts that thing bb");

        let program = utils::link_program(&gl,
            vert::first::SHADER,
            frag::color_from_texture::SHADER,
            
        ).expect("failed to link shader program");

//       use set_program
        gl.use_program(Some(&program));

        
        // let image = ;

        let vert_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buffer));
        unsafe{
            let vert_array = js_sys::Float32Array::view(&[      
                -1.0, -1.0, 0.0,
                 1.0, -1.0, 0.0,
                
                -1.0, 1.0, 0.0,
                 1.0, 1.0, 0.0
            ]);

            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        let vert_pos_loc : u32 = gl.get_attrib_location(&program, &"a_vertexPosition") as u32;
        gl.vertex_attrib_pointer_with_i32(vert_pos_loc, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(vert_pos_loc);

        let tri_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&tri_buffer));
        unsafe {
            let tri_array = js_sys::Uint16Array::view(&[
                0, 1, 2,
                2, 1, 3
            ]);

            gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &tri_array, GL::STATIC_DRAW);
        }

        

        let tex_coord_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));
        

        unsafe {
            let uv_arr = js_sys::Float32Array::view(&[
                0., 1.,
                1., 1.,
                0., 0.,
                1., 0.
            ]);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &uv_arr, GL::STATIC_DRAW);
        }

        let tex_coord_location : u32 = gl.get_attrib_location(&program, &"a_texCoord") as u32;
        gl.vertex_attrib_pointer_with_i32(tex_coord_location, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(tex_coord_location);

        let tint_loc = gl.get_uniform_location(&program, &"u_tint").unwrap();
        gl.uniform4fv_with_f32_array(Some(&tint_loc), &[1., 0., 0., 1.]);

        
        let tex = gl.create_texture().unwrap();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&tex));
        gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA ,GL::UNSIGNED_BYTE, &img).unwrap();
        
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

        gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
        console_log!("maybe image onscreen now");     



        Self{
            program: program
        }
    }

    pub fn render(&self, gl: GL){

    }
}




// stolen code from https://users.rust-lang.org/t/a-future-for-loading-images-via-web-sys/42370/2
// seems easiest way to do it is by stealing so lest go

// pub struct ImageFuture {
//     image: Option<HtmlImageElement>,
//     load_failed: Rc<Cell<bool>>,
// }

// impl ImageFuture {
//     pub fn new(path: &str) -> Self {
//         let image = HtmlImageElement::new().unwrap();
//         image.set_src(path);
//         ImageFuture {
//             image: Some(image),
//             load_failed: Rc::new(Cell::new(false)),
//         }
//     }
// }

// impl Future for ImageFuture {
//     type Output = Result<HtmlImageElement, ()>;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match &self.image {
//             Some(image) if image.complete() => {
//                 let image = self.image.take().unwrap();
//                 let failed = self.load_failed.get();

//                 if failed {
//                     Poll::Ready(Err(()))
//                 } else {
//                     Poll::Ready(Ok(image))
//                 }
//             }
//             Some(image) => {
//                 let waker = cx.waker().clone();
//                 let on_load_closure = Closure::wrap(Box::new(move || {
//                     waker.wake_by_ref();
//                 }) as Box<dyn FnMut()>);
//                 image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
//                 on_load_closure.forget();

//                 let waker = cx.waker().clone();
//                 let failed_flag = self.load_failed.clone();
//                 let on_error_closure = Closure::wrap(Box::new(move || {
//                     failed_flag.set(true);
//                     waker.wake_by_ref();
//                 }) as Box<dyn FnMut()>);
//                 image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
//                 on_error_closure.forget();

//                 Poll::Pending
//             }
//             _ => Poll::Ready(Err(())),
//         }
//     }
// }