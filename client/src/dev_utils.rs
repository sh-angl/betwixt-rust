use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

struct toilet<T: std::fmt::Debug>{
    pub pieces: Vec<T>
}

// fn shitter<T>(shit: T) -> (&JsValue, bool){
//     let yes: &JsValue = &"4".into();
//     (yes, true)
// }

#[macro_export(console_log)]
macro_rules! console_log {
    ($($t:tt)*) => (unsafe{crate::dev_utils::log(&format_args!($($t)*).to_string())})
}

