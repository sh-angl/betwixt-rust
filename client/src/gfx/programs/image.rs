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
            vert::texCords::SHADER,
            frag::color_from_texture::SHADER,
            
        ).expect("failed to link shader program");

//       use set_program
        gl.use_program(Some(&program));

        
        // let image = ;
        let tex_coord_location : u32 = gl.get_attrib_location(&program, &"a_texCoord") as u32;

        let tex_coord_buffer = gl.create_buffer().unwrap();

        let optionated: Option<&WebGlBuffer> = Some(&tex_coord_buffer);

        gl.bind_buffer(GL::ARRAY_BUFFER, optionated);

        unsafe{
            let vert_array = js_sys::Float32Array::view(&[      
                0.0,  0.0,
                1.0,  0.0,
                0.0,  1.0,
                0.0,  1.0,
                1.0,  0.0,
                1.0,  1.0]);

            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }

        gl.enable_vertex_attrib_array(tex_coord_location);

        gl.vertex_attrib_pointer_with_i32(tex_coord_location, 2, GL::FLOAT, false, 0, 0);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

        gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA ,GL::UNSIGNED_BYTE, &img).unwrap();

        let primitive_type = GL::TRIANGLES;
        let offset = 0;
        let count = 0;
        gl.draw_arrays(primitive_type, offset, count); 
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