use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod course;
mod utlis;
#[derive(Serialize, Deserialize)]
pub struct JSONCourseList {
    pub courses: Vec<JSONCourse>,
}
#[derive(Serialize, Deserialize)]
pub struct JSONCourse {
    pub title: String,
    pub code: String,
    pub uoc: u8,
    pub level: u8,
    pub description: String,
    pub study_level: String,
    pub school: String,
    pub faculty: String,
    pub campus: String,
    pub equivalents: HashMap<String, u8>,
    pub exclusions: HashMap<String, u8>,
    pub terms: Vec<String>,
    pub gen_ed: bool,
    pub raw_requirements: String,
    pub is_multiterm: bool,
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn getProgramInfo(code: String) {

}
#[wasm_bindgen]
pub fn getCourseInfo(code: String) {
    
}



fn main() {
    println!("Hello, world!");
}
