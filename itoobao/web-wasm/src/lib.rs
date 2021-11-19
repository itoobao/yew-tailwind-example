#![recursion_limit = "1024"]
mod app;
mod components;
mod error;
mod pages;
mod routes;
mod services;
mod switch;
mod types;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
