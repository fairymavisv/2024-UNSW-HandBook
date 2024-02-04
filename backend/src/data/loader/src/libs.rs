/// This module contains the implementation of the `Course` struct and the `CourseManager` struct.
/// The `Course` struct represents a course with its properties such as title, code, UOC (Units of Credit), description, level, study level, offering terms, campus, and requirements.
/// The `CourseManager` struct manages a collection of courses, equivalent courses, and exclusion courses.
/// It provides methods to parse course data from JSON files, retrieve courses by code, and access the course collection.
pub mod course;

/// This module is responsible for handling program and specialisation data
/// It provides a `ProgramManager` struct to manage the data and provide access to the data
/// It also provides a set of structs to represent the data
pub mod program;

/// This module is used to parse the requirements string and evaluate the requirements
/// This module provides the `Requirements` struct and the `Node` trait
pub mod requirements;

/// This module provides search functionality for program and course
/// The main purpose of this module is to provide a way to search for courses that are eligible for a program, and list program structure
pub mod search;

/// This module contains the definition of the `CourseCode`, `ProgramCode`, `OfferingTerm`, `Campus`, and `StudyLevel`
pub mod utlis;

/// This module is the implementation of the handbook data interface
/// It provides the wasm_bindgen interface for the typescript to access the handbook data

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use course::CourseManager;
use program::{Program, ProgramManager};
use std::{
    collections::HashMap,
    path::Component,
    rc::Rc,
    sync::{Arc, RwLock},
};
use wasm_bindgen::prelude::*;
use crate::utlis::{CourseCode, ProgramCode};
// #[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

/// Test function to check if the wasm is working
#[wasm_bindgen]
pub fn test_io(input: &str) -> String {
    String::from("Hello World!")
}

/// The HandbookDataInterface is the main interface for the typescript to access the handbook data
#[wasm_bindgen]
pub struct HandbookDataInterface {
    programs: Arc<RwLock<ProgramManager>>,
    courses: Arc<RwLock<CourseManager>>,
}

/// The JsCourseInfo is the course information struct that is used to pass course information to the typescript
/// It is also used to convert the course information from the rust to the typescript
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
/// This is the implementation of the JsCourseInfo, which is not directly used by the typescript
impl JsCourseInfo {
    /// Get the course code
    pub fn code(&self) -> String {
        self.code.clone()
    }

    /// Get the course name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the course uoc
    pub fn uoc(&self) -> u8 {
        self.uoc
    }

    /// Get the course description
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Get the course conditions
    pub fn conditions(&self) -> String {
        self.conditions.clone()
    }

    /// Get the course offerings
    pub fn offerings(&self) -> Vec<String> {
        self.offerings.clone()
    }
}

/// The JsProgramInfo is the program information struct that is used to pass program information to the typescript
/// It is also used to convert the program information from the rust to the typescript
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

/// The JsProgramStructure is the program structure struct that is used to pass program structure information to the typescript
/// It is also used to convert the program structure information from the rust to the typescript
#[derive(Tsify, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsProgramStructure {
    course_list: Vec<(String, Vec<String>)>,
    specialisation_list: Vec<(String, Vec<String>)>,
}

/// This is the implementation of the JsProgramStructure, which is not directly used by the typescript
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
/// This is the implementation of the JsProgramInfo, which is not directly used by the typescript
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
/// The JsSpecialisationInfo is the specialisation information struct that is used to pass specialisation information to the typescript
/// It is also used to convert the specialisation information from the rust to the typescript
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsSpecialisationInfo {
    name: String,
    code: String,
    uoc: String,
}

/// This is the implementation of the JsSpecialisationInfo, which is not directly used by the typescript
impl JsProgramInfo {
    /// Convert the program information from the Program to JsProgramInfo
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
    /// Convert the course information from the course::Course to JsCourseInfo
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

/// The HandbookDataInterface implementation
#[wasm_bindgen]
impl HandbookDataInterface {
    /// Create a new HandbookDataInterface
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    /// * `data_src_path` - The path to the data source
    /// 
    /// # Returns
    /// 
    /// The HandbookDataInterface
    /// 
    /// # Example
    /// 
    /// ```
    /// let data_src_path = "data";
    /// let handbook_data_interface = HandbookDataInterface::new(data_src_path);
    /// ```
    /// 
    /// # Panics
    /// 
    /// If the data source is not found, the function will panic
    /// 
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

    /// Get the course information
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    /// * `code` - The course code
    /// 
    /// # Returns
    /// 
    /// The JsCourseInfo
    /// None if the course code is not found, or the course code is invalid
    /// 
    /// If the course code is not found, the function will print the error message
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = "COMP1511";
    /// let course_info = handbook_data_interface.get_course_info(code);
    /// ```
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

    /// Get the program information
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    ///  * `code` - The program code
    /// 
    /// # Returns
    /// 
    /// The JsProgramInfo
    /// None if the program code is not found, or the program code is invalid
    /// 
    /// If the program code is not found, the function will print the error message
    /// 
    /// All specialisation codes will be included in the structure field
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = "3778";
    /// let program_info = handbook_data_interface.get_program_info(code);
    /// ```
    /// 
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

    /// Get the program and specialisation information
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    /// * `code` - The program code
    /// * `spec` - The specialisation codes, if any. 
    /// it could be None, or a list of specialisation codes (major, minor, honours)
    /// If None, the function will return program information and all detailed specialisation information
    /// 
    /// # Returns
    /// 
    /// The JsProgramInfo
    /// None if the program code is not found, or the program code is invalid
    /// 
    /// If the program code is not found, the function will print the error message
    /// 
    /// Only specialisations that given specialisation codes will be included in the structure field
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = "3778";
    /// let spec = vec!["COMPA1", "ACCTA1"];
    /// let program_info = handbook_data_interface.get_program_and_spec_info(code, spec);
    /// ```
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

    /// Get the specialisation information
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    /// * `code` - The program code
    /// 
    /// # Returns
    /// 
    /// The List of Specialisation Code
    /// None if the program code is not found, or the program code is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = "3778";
    /// let specialisation_info = handbook_data_interface.get_specialisation_info(code);
    /// ```
    /// 
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

    /// Get the list of eligible courses
    /// * wasm_bindgen is used to expose the function to the typescript
    /// 
    /// # Arguments
    /// 
    /// * `program_code` - The program code
    /// * `taken_course` - The list of taken course codes
    /// * `wam` - The weighted average mark
    /// 
    /// # Returns
    /// 
    /// The List of Eligible Course Code
    /// None if the program code is not found, or the program code is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// let program_code = "3778";
    /// let taken_course = vec!["COMP1511", "COMP1521"];
    /// let wam = Some(75);
    /// let eligible_courses = handbook_data_interface.list_eligible_courses(program_code, taken_course, wam);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Course code parsing error
    /// 
    pub fn list_eligible_courses(
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
            .list_eligible_courses(&pool, &program_code, &taken_course, &wam)
            .into_par_iter()
            .map(|course| course.code())
            .collect();
        Some(result)
    }
}
