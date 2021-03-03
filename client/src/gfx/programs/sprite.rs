use nalgebra::{Matrix4, Translation3, Vector3};
use wasm_bindgen::{JsCast};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::Promise;
use wasm_bindgen_futures::*;
use wasm_bindgen::prelude::*;

use crate::camera::Camera;

use super::super::{utils};
use super::super::shaders::{frag, vert};
// imports for stolen imageFuture code
use futures::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use std::cell::Cell;
use std::rc::Rc;

pub struct Sprite {
    // vertices: &'static js_sys::Float32Array,
    // triangles: &'static js_sys::Uint16Array,
    // uvs: &'static js_sys::Float32Array,

    program: WebGlProgram,
    vertex_buf: WebGlBuffer,
    triangle_buf: WebGlBuffer,
    uv_buf: WebGlBuffer,
    texture: WebGlTexture,
}



impl Sprite {
    pub fn init(gl: &GL) {
        // TODO: move vert/tri/uv buffer to statics and init them here
    }
    pub fn new(gl: &GL, img: HtmlImageElement) -> Self{
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


        
        let tex = gl.create_texture().unwrap();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&tex));
        gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA ,GL::UNSIGNED_BYTE, &img).unwrap();
        
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);



        Self{
            program: program,
            vertex_buf: vert_buffer,
            triangle_buf: tri_buffer,
            uv_buf: tex_coord_buffer,
            texture: tex,
        }
    }

    pub fn render(&self, gl: &GL, camera: &Camera, position: &Vector3<f32>){
        gl.use_program(Some(&self.program));
        
        // vertex params
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buf));
        let vert_pos_loc : u32 = gl.get_attrib_location(&self.program, &"a_vertexPosition") as u32;
        gl.vertex_attrib_pointer_with_i32(vert_pos_loc, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(vert_pos_loc);
        
        // uv params
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.uv_buf));
        let tex_coord_location : u32 = gl.get_attrib_location(&self.program, &"a_texCoord") as u32;
        gl.vertex_attrib_pointer_with_i32(tex_coord_location, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(tex_coord_location);

        // tint uniform
        let tint_loc = gl.get_uniform_location(&self.program, &"u_tint").unwrap();
        gl.uniform4fv_with_f32_array(Some(&tint_loc), &[1., 1., 1., 1.]);

        // model proj matrix
        let model_loc = gl.get_uniform_location(&self.program, &"u_mMatrix").unwrap();
        let model = Translation3::new(position.x, position.y, position.z);
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(Some(&model_loc), false, &model_array);

        
        // view matrix
        let loc = gl.get_uniform_location(&self.program, &"u_vMatrix").unwrap();
        gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &camera.view());

        // projection matrix
        let loc = gl.get_uniform_location(&self.program, &"u_pMatrix").unwrap();
        gl.uniform_matrix4fv_with_f32_array(Some(&loc), false, &camera.projection());

        // triangle params
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.triangle_buf));
        
        // tex params
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));

        // actually draw
        gl.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
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