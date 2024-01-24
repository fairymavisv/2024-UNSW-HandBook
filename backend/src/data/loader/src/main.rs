use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use wasm_bindgen::prelude::*;

// mod course;
// mod requirements;
// mod utlis;
// mod program;
// mod search;

// extern crate cfg_if;
// extern crate wasm_bindgen;
// use cfg_if::cfg_if;
// use wasm_bindgen::prelude::*;

mod course;
mod program;
mod requirements;
mod search;
mod utlis;

// cfg_if! {
//     // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//     // allocator.
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// pub fn getProgramInfo(code: String) {

// }
// #[wasm_bindgen]
// pub fn getCourseInfo(code: String) {

// }
// extern crate cfg_if;
// extern crate wasm_bindgen;

// use cfg_if::cfg_if;
// use wasm_bindgen::prelude::*;

// cfg_if! {
//     // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//     // allocator.
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }

fn main() {
    println!("Hello, world!");
}
