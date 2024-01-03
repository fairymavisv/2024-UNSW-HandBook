use std::{collections::HashMap, fmt::Formatter};
use std::fmt::Display;
use crate::utlis::{CourseCode, ProgramCode};
use crate::{utlis::{StudyLevel, Campus, OfferingTerm}, JSONCourse, JSONCourseList, requirements::{Requirements}};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};
use serde_json::{Value};
use std::fs;

pub enum ExclusionCondition {
    Course(CourseCode),
    Program(ProgramCode),
    PlainText(String),
}


pub struct Course {
    title: String,
    code: CourseCode,
    uoc: u8,
    level: u8,
    study_level: StudyLevel,
    offering_terms: Vec<OfferingTerm>,
    campus: Campus,
    requirements: Option<Requirements>,
} 

impl Course {
    pub fn new(title: String, code: String, uoc: u8, level: u8, study_level: StudyLevel, offering_terms: Vec<OfferingTerm>, campus: Campus, requirements: Option<Requirements>) -> Course {
        Course {
            title,
            code: CourseCode::from_str(&code).expect(format!("Invalid course code: {}", code).as_str()),
            uoc,
            level,
            study_level,
            offering_terms,
            campus,
            requirements
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn code(&self) -> String {
        self.code.to_string()
    }
    pub fn uoc(&self) -> &u8 {
        &self.uoc
    }
    pub fn level(&self) -> &u8 {
        &self.level
    }
    pub fn study_level(&self) -> &StudyLevel {
        &self.study_level
    }
    pub fn offering_terms(&self) -> &Vec<OfferingTerm> {
        &self.offering_terms
    }
    pub fn campus(&self) -> &Campus {
        &self.campus
    }
    pub fn requirements(&self) -> &Option<Requirements> {
        &self.requirements
    }

}

pub struct CourseManager {
    courses: HashMap<String, Course>,
    equivalent_courses: HashMap<String, Vec<String>>,
    exclusion_courses: HashMap<String, Vec<ExclusionCondition>>,
}

impl CourseManager {
    pub fn new(courses: JSONCourseList, equivalent_path: &str, exclusion_path: &str) -> CourseManager {
        let courses = CourseManager::parse_from_courses_json(courses);
        let equivalent_courses = CourseManager::parse_from_equivalent_json(equivalent_path);
        let exclusion_courses = CourseManager::parse_from_exclusion_json(exclusion_path);
        CourseManager {
            courses,
            equivalent_courses,
            exclusion_courses,
        }

    }
    fn parse_from_courses_json(json_courses: JSONCourseList) -> HashMap<String, Course> {
        let course_list: HashMap<String, Course> = json_courses.courses.into_par_iter().map(|json_course| {
            let title = json_course.title;
            let code = json_course.code;
            let uoc = json_course.uoc;
            let level = json_course.level;
            let study_level = StudyLevel::from_str(&json_course.study_level).expect(format!("Unexpected study level: {}", json_course.study_level).as_str());
            let offering_terms = json_course.terms.iter().filter_map(|term| OfferingTerm::from_str(term)).collect::<Vec<OfferingTerm>>();
            let campus = Campus::from_str(&json_course.campus).expect(format!("Unexpected campus: {}", json_course.campus).as_str());
            // let requirements = Requirements::parse(Requirements::tokenize(&json_course.raw_requirements));
            
            Course::new(title, code, uoc, level, study_level, offering_terms, campus, Requirements::try_new(json_course.raw_requirements))
        }).map(|course| (course.code.to_string(), course)).collect();
        course_list
    }
    fn parse_from_equivalent_json(path: &str) -> HashMap<String, Vec<String>> {
        let json = fs::read_to_string(path).expect("Unable to read equivalent course json file");
        let mut lookup: HashMap<String, Value> = serde_json::from_str(&json).expect("Json parsing failed");
        lookup.retain(|key, value| {
            if value.is_object() {
                value.as_object().unwrap().keys().len() > 0
            } else {
                false
            }
        });
        lookup.into_iter().map(|(string, value)| {
            let mut vec: Vec<String> = Vec::new();
            if value.is_object() {
                let object = value.as_object().unwrap();
                for (key, _) in object {
                    vec.push(key.clone());
                }
            }
            (string, vec)
        }).collect::<HashMap<String, Vec<String>>>()
    }
    fn parse_from_exclusion_json(path: &str) -> HashMap<String, Vec<ExclusionCondition>> {
        let json = fs::read_to_string(path).expect("Unable to read exclusion course json file");
        let mut lookup: HashMap<String, Value> = serde_json::from_str(&json).expect("Json parsing failed");
        lookup.retain(|key, value| {
            if value.is_object() {
                value.as_object().unwrap().keys().len() > 0
            } else {
                false
            }
        });
        lookup.into_iter().map(|(string, value)| {
            let mut vec: Vec<ExclusionCondition> = Vec::new();
            if value.is_object() {
                let object = value.as_object().unwrap();
                for (key, value) in object {
                    if CourseCode::is_code(&key) {
                        vec.push(ExclusionCondition::Course(CourseCode::from_str(&key).unwrap()));
                    } else if ProgramCode::is_code(&key) {
                        vec.push(ExclusionCondition::Program(ProgramCode::from_str(&key).unwrap()));
                    } else {
                        vec.push(ExclusionCondition::PlainText(value.as_str().unwrap_or("Please check handbook for further detail").to_string()));
                    }
                }
            }
            (string, vec)
        }).collect::<HashMap<String, Vec<ExclusionCondition>>>()
    }

    


}