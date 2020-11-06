use wasm_bindgen;
use wasm_bindgen::prelude::*; 
use wasm_bindgen::JsCast;
use web_sys:: {WebSocket, MessageEvent, ErrorEvent};

#[macro_use]
mod dev_utils;
mod gfx;

mod controls;



#[wasm_bindgen(start)]
pub fn initialise() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    controls::init_controls(&document);
    gfx::utils::init_gfx(&document);
    




    console_log!("yeah");



    // unsafe{
    //     console_log!("yeah, rust bb");
    // }
    

    // let ws = WebSocket::new("ws://127.0.0.1:8080")?;
    
    // ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    // // stolen from the docs bb


    // let cloned_ws = ws.clone();
    // let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
    //     if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
    //         console_log!("message event, received arraybuffer: {:?}", abuf);
    //         let array = js_sys::Uint8Array::new(&abuf);
    //         let len = array.byte_length() as usize;
    //         console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());

    //         cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
    //         match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
    //             Ok(_) => console_log!("binary message successfully sent"),
    //             Err(err) => console_log!("error sending message: {:?}", err),
    //         }
    //     } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
    //         console_log!("message event, received Text: {:?}", txt);
    //     } else {
    //         console_log!("message event, received Unknown: {:?}", e.data());
    //     }
    // })as Box<dyn FnMut(MessageEvent)>);

    // ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

    // onmessage_callback.forget();

    // let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
    //     console_log!("error event: {:?}", e);
    // }) as Box<dyn FnMut(ErrorEvent)>);
    // ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    // onerror_callback.forget();
    
    // let cloned_ws = ws.clone();
    // let onopen_callback = Closure::wrap(Box::new(move |_| {
    //     console_log!("socket opened");
    //     match cloned_ws.send_with_str("ping") {
    //         Ok(_) => console_log!("message successfully sent"),
    //         Err(err) => console_log!("error sending message: {:?}", err),
    //     }
    //     // send off binary message
    //     match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
    //         Ok(_) => console_log!("binary message successfully sent"),
    //         Err(err) => console_log!("error sending message: {:?}", err),
    //     }
    // }) as Box<dyn FnMut(JsValue)>);
    // ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    // onopen_callback.forget();

    
    Ok(())
}
