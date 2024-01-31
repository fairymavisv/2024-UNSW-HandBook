// extern crate cfg_if;
// extern crate wasm_bindgen;
// use cfg_if::cfg_if;
// use wasm_bindgen::prelude::*;

mod course;
mod program;
mod requirements;
mod search;
mod utlis;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
// use serde_wasm_bindgen;
use course::CourseManager;
use program::{Program, ProgramManager};
use std::{
    collections::HashMap,
    path::Component,
    rc::Rc,
    sync::{Arc, RwLock},
};
use wasm_bindgen::prelude::*;
// use tokio::
use crate::utlis::{CourseCode, ProgramCode};
// #[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn test_io(input: &str) -> String {
    String::from("Hello World!")
}

#[wasm_bindgen]
pub struct HandbookDataInterface {
    programs: Arc<RwLock<ProgramManager>>,
    courses: Arc<RwLock<CourseManager>>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsCourseInfo {
    code: String,
    name: String,
    uoc: u8,
    description: String,
    conditions: String,
    offerings: Vec<String>,
}
impl JsCourseInfo {
    pub fn code(&self) -> String {
        self.code.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn uoc(&self) -> u8 {
        self.uoc
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn conditions(&self) -> String {
        self.conditions.clone()
    }
    pub fn offerings(&self) -> Vec<String> {
        self.offerings.clone()
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsProgramInfo {
    name: String,
    code: String,
    uoc: String,
    overview: String,
    structure_summary: String,
    structure: JsProgramStructure,
}
#[derive(Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsProgramStructure {
    course_list: Vec<(String, Vec<String>)>,
    specialisation_list: Vec<(String, Vec<String>)>,
}
impl JsProgramStructure {
    pub fn new(
        course_list: Vec<(String, Vec<String>)>,
        specialisation_list: Vec<(String, Vec<String>)>,
    ) -> Self {
        JsProgramStructure {
            course_list,
            specialisation_list,
        }
    }
    pub fn empty() -> Self {
        JsProgramStructure {
            course_list: Vec::new(),
            specialisation_list: Vec::new(),
        }
    }
    pub fn from_struture(structure: Vec<(String, Vec<String>)>) -> Self {
        let course_list: Vec<(String, Vec<String>)> = structure
            .iter()
            .filter(|(name, _)| {
                !(name.starts_with("Major -")
                    || name.starts_with("Minor -")
                    || name.starts_with("Honours -"))
            })
            .map(|entry| entry.clone())
            .collect();
        let specialisation_list: Vec<(String, Vec<String>)> = structure
            .into_iter()
            .filter(|(name, _)| {
                (name.starts_with("Major -")
                    || name.starts_with("Minor -")
                    || name.starts_with("Honours -"))
            })
            .collect();
        JsProgramStructure {
            course_list: course_list,
            specialisation_list: specialisation_list,
        }
    }
}
impl JsProgramInfo {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn code(&self) -> String {
        self.code.clone()
    }
    pub fn uoc(&self) -> String {
        self.uoc.clone()
    }
    pub fn overview(&self) -> String {
        self.overview.clone()
    }
    pub fn structure_summary(&self) -> String {
        self.structure_summary.clone()
    }
    pub fn structure(&self) -> JsProgramStructure {
        self.structure.clone()
    }
}
#[wasm_bindgen]
pub struct JsSpecialisationInfo {
    name: String,
    code: String,
    uoc: String,
}
impl JsProgramInfo {
    fn from(program: &Program) -> Self {
        JsProgramInfo {
            name: program.title().to_string(),
            code: program.code().to_string(),
            uoc: program.uoc().to_string(),
            overview: program.overview().to_string(),
            structure_summary: program.structure_summary().to_string(),
            structure: JsProgramStructure::empty(),
        }
    }
}

impl From<course::Course> for JsCourseInfo {
    fn from(course: course::Course) -> Self {
        JsCourseInfo {
            code: course.code().to_string(),
            name: course.title().to_string(),
            uoc: course.uoc(),
            description: course.description().to_string(),
            conditions: match course.requirements() {
                Some(requirements) => requirements.to_string(),
                None => "Please Report Bug: Course condition parsing error".to_string(),
            },
            offerings: course
                .offering_terms()
                .iter()
                .map(|offering| offering.to_string())
                .collect(),
        }
    }
}

#[wasm_bindgen]
impl HandbookDataInterface {
    pub fn new(data_src_path: &str) -> Self {
        let programs = ProgramManager::new(
            format!("{}/programsProcessed.json", data_src_path).as_str(),
            format!("{}/specialisationsProcessed.json", data_src_path).as_str(),
        );
        let courses = CourseManager::new(
            format!("{}/coursesProcessed.json", data_src_path).as_str(),
            format!("{}/equivalents.json", data_src_path).as_str(),
            format!("{}/exclusions.json", data_src_path).as_str(),
        );
        HandbookDataInterface {
            programs: Arc::new(RwLock::new(programs)),
            courses: Arc::new(RwLock::new(courses)),
        }
    }

    pub fn get_course_info(&self, code: &str) -> Option<JsCourseInfo> {
        let courses = self.courses.read().unwrap();
        let course = courses.get_course(&CourseCode::from_str(code)?);
        match course {
            Ok(course) => Some(JsCourseInfo::from(course.clone())),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub fn get_program_info(&self, code: &str) -> Option<JsProgramInfo> {
        let programs = self.programs.read().unwrap();
        let program_code = ProgramCode::from_str(code)?;
        let program = programs.get_program(&program_code);
        match program {
            Ok(program) => {
                let structure = programs
                    .get_program_structure(&program_code, false, None)
                    .unwrap_or(Vec::new());
                let mut program = JsProgramInfo::from(program);
                program.structure = JsProgramStructure::from_struture(structure);
                Some(program)
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
    pub fn get_program_and_spec_info(
        &self,
        code: &str,
        spec: Option<Vec<String>>,
    ) -> Option<JsProgramInfo> {
        let programs = self.programs.read().unwrap();
        let program_code = ProgramCode::from_str(code)?;
        let program = programs.get_program(&program_code);
        match program {
            Ok(program) => {
                let structure = programs
                    .get_program_structure(&program_code, true, spec.as_ref())
                    .unwrap_or(Vec::new());
                let mut program = JsProgramInfo::from(program);
                program.structure = JsProgramStructure::from_struture(structure);
                Some(program)
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    pub fn list_program_all_coursecodes(&self, program_code: &str) -> Option<Vec<String>> {
        let courses = self.courses.read().ok()?;
        let programs = self.programs.read().ok()?;
        let program_code = ProgramCode::from_str(program_code)?;
        let pool = programs.get_course_pool(&program_code).ok()?;
        let result: Vec<String> = courses
            .list_courses_from_pool(&pool)
            .into_par_iter()
            .map(|course| course.code())
            .collect();
        Some(result)
    }

    pub fn list_eligable_courses(
        &self,
        program_code: &str,
        taken_course: Vec<String>,
        wam: Option<u8>,
    ) -> Option<Vec<String>> {
        let courses = self.courses.read().ok()?;
        let programs = self.programs.read().ok()?;
        let program_code = ProgramCode::from_str(program_code)?;
        let pool = programs.get_course_pool(&program_code).ok()?;
        let result: Vec<String> = courses
            .list_eligable_courses(&pool, &program_code, &taken_course, &wam)
            .into_par_iter()
            .map(|course| course.code())
            .collect();
        Some(result)
    }
}
