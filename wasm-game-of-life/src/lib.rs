mod app;
mod component_designer;
mod dsc;
mod polygon;
mod sketch;
mod system;
mod utils;

use app::{App, AppView};

use js_sys::Function;
use sodium_rust::Listener;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, wasm-game-of-life!".into()
}

pub struct UnsafeSendSync<A> {
    pub value: A,
}

unsafe impl<A> Send for UnsafeSendSync<A> {}
unsafe impl<A> Sync for UnsafeSendSync<A> {}

#[wasm_bindgen]
pub fn new_app() -> *mut App {
    Box::into_raw(Box::new(App::new()))
}

#[wasm_bindgen]
pub fn drop_app(app: *mut App) {
    unsafe {
        Box::from_raw(app);
    };
}

#[wasm_bindgen]
pub fn app_c_current_view_listen(app: *mut App, callback: Function) -> *mut Listener {
    let app = unsafe { &mut *app };
    let callback = UnsafeSendSync { value: callback };
    let listener = app.c_current_view.listen_weak(move |app_view: &AppView| {
        match app_view {
            AppView::LogInOrRegister => {
                callback
                    .value
                    .call1(&JsValue::UNDEFINED, &JsValue::from_f64(1f64));
            }
            AppView::ComponentDesigner(_) => {
                callback
                    .value
                    .call1(&JsValue::UNDEFINED, &JsValue::from_f64(2f64));
            }
        };
    });
    Box::into_raw(Box::new(listener))
}

#[wasm_bindgen]
pub fn app_log_in(app: *mut App) {
    let app = unsafe { &mut *app };
    app.log_in();
}

#[wasm_bindgen]
pub fn drop_listener(listener: *mut Listener) {
    unsafe {
        Box::from_raw(listener);
    };
}
