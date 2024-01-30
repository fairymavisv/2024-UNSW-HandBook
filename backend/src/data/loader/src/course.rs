use crate::utlis::{CourseCode, ProgramCode};
use crate::{
    requirements::Requirements,
    utlis::{Campus, OfferingTerm, StudyLevel},
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use std::fs;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Formatter};

#[derive(Serialize, Deserialize)]
pub struct JSONCourseList {
    pub courses: HashMap<String, JSONCourse>,
}

impl JSONCourseList {
    pub fn new(courses: HashMap<String, JSONCourse>) -> Self {
        JSONCourseList { courses }
    }
}
#[derive(Serialize, Deserialize)]
pub struct JSONCourse {
    pub title: String,
    pub code: String,
    pub UOC: u8,
    pub level: u8,
    pub description: String,
    pub study_level: String,
    pub school: String,
    pub faculty: String,
    pub campus: String,
    pub equivalents: HashMap<String, Value>,
    pub exclusions: HashMap<String, Value>,
    pub terms: Vec<String>,
    pub gen_ed: bool,
    pub raw_requirements: String,
    pub is_multiterm: bool,
}

pub enum ExclusionCondition {
    Course(CourseCode),
    Program(ProgramCode),
    PlainText(String),
}

// impl  {

// }

#[derive(Clone)]
pub struct Course {
    title: String,
    code: CourseCode,
    uoc: u8,
    description: String,
    level: u8,
    study_level: StudyLevel,
    offering_terms: Vec<OfferingTerm>,
    campus: Campus,
    requirements: Option<Arc<Requirements>>,
}

impl Course {
    pub fn new(
        title: String,
        code: String,
        uoc: u8,
        description: String,
        level: u8,
        study_level: StudyLevel,
        offering_terms: Vec<OfferingTerm>,
        campus: Campus,
        requirements: Option<Arc<Requirements>>,
    ) -> Course {
        Course {
            title,
            code: CourseCode::from_str(&code)
                .expect(format!("Invalid course code: {}", code).as_str()),
            uoc,
            description,
            level,
            study_level,
            offering_terms,
            campus,
            requirements,
        }
    }

    pub fn available_at_term(&self, offering_terms: &OfferingTerm) -> bool {
        self.offering_terms.contains(offering_terms)
    }

    pub fn is_eligable(
        &self,
        program_code: &ProgramCode,
        taken_course: &Vec<String>,
        wam: &Option<u8>,
        course_manager: &CourseManager,
    ) -> Result<bool, String> {
        if self.requirements.is_none() {
            println!("Warning: Requirements parsing error was happen");
            Ok(true)
        } else {
            self.requirements.as_ref().unwrap().is_satisified(
                program_code,
                taken_course,
                wam,
                course_manager,
            )
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn code(&self) -> String {
        self.code.to_string()
    }
    pub fn uoc(&self) -> u8 {
        self.uoc
    }
    pub fn description(&self) -> &str {
        &self.description
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
    pub fn requirements(&self) -> Option<Arc<Requirements>> {
        self.requirements.as_ref().map(|x| x.clone())
    }

}

pub struct CourseManager {
    courses: HashMap<String, Course>,
    equivalent_courses: HashMap<String, Vec<String>>,
    exclusion_courses: HashMap<String, Vec<ExclusionCondition>>,
}

impl CourseManager {
    pub fn new(courses_path: &str, equivalent_path: &str, exclusion_path: &str) -> CourseManager {
        let courses = CourseManager::parse_from_courses_json(courses_path);
        let equivalent_courses = CourseManager::parse_from_equivalent_json(equivalent_path);
        let exclusion_courses = CourseManager::parse_from_exclusion_json(exclusion_path);
        CourseManager {
            courses,
            equivalent_courses,
            exclusion_courses,
        }
    }
    fn parse_from_courses_json(json_path: &str) -> HashMap<String, Course> {
        let json = fs::read_to_string(json_path).expect("Unable to read course json file");
        let json_courses: HashMap<String, JSONCourse> = serde_json::from_str(&json).unwrap();
        let json_courses = JSONCourseList::new(json_courses);
        let course_list: HashMap<String, Course> = json_courses
            .courses
            // .into_iter()
            .into_par_iter()
            .map(|(course_name, json_course)| {
                // println!("Parsing for course {}: ", course_name);
                let title = json_course.title;
                let code = json_course.code;
                let uoc = json_course.UOC;
                let level = json_course.level;
                let study_level = StudyLevel::from_str(&json_course.study_level).expect(
                    format!("Unexpected study level: {}", json_course.study_level).as_str(),
                );
                let offering_terms = json_course
                    .terms
                    .iter()
                    .filter_map(|term| OfferingTerm::from_str(term))
                    .collect::<Vec<OfferingTerm>>();
                let campus = Campus::from_str(&json_course.campus)
                    .expect(format!("Unexpected campus: {}", json_course.campus).as_str());
                let mut requirements = Requirements::try_new(&json_course.raw_requirements);
                match &campus {
                    Campus::Canberra => requirements = None,
                    _ => (),
                }
                if json_course.school.to_string().eq("UNSW Global") {
                    requirements = None
                }
                let description = json_course.description;
                Course::new(
                    title,
                    code,
                    uoc,
                    description,
                    level,
                    study_level,
                    offering_terms,
                    campus,
                    match requirements {
                        Some(requirements) => Some(Arc::new(requirements)),
                        None => None,
                    },
                )
            })
            .map(|course| (course.code.to_string(), course))
            .collect();
        course_list
    }
    fn parse_from_equivalent_json(path: &str) -> HashMap<String, Vec<String>> {
        let json = fs::read_to_string(path).expect("Unable to read equivalent course json file");
        let mut lookup: HashMap<String, Value> =
            serde_json::from_str(&json).expect("Json parsing failed");
        lookup.retain(|key, value| {
            if value.is_object() {
                value.as_object().unwrap().keys().len() > 0
            } else {
                false
            }
        });
        lookup
            .into_iter()
            .map(|(string, value)| {
                let mut vec: Vec<String> = Vec::new();
                if value.is_object() {
                    let object = value.as_object().unwrap();
                    for (key, _) in object {
                        vec.push(key.clone());
                    }
                }
                (string, vec)
            })
            .collect::<HashMap<String, Vec<String>>>()
    }
    fn parse_from_exclusion_json(path: &str) -> HashMap<String, Vec<ExclusionCondition>> {
        let json = fs::read_to_string(path).expect("Unable to read exclusion course json file");
        let mut lookup: HashMap<String, Value> =
            serde_json::from_str(&json).expect("Json parsing failed");
        lookup.retain(|key, value| {
            if value.is_object() {
                value.as_object().unwrap().keys().len() > 0
            } else {
                false
            }
        });
        lookup
            .into_iter()
            .map(|(string, value)| {
                let mut vec: Vec<ExclusionCondition> = Vec::new();
                if value.is_object() {
                    let object = value.as_object().unwrap();
                    for (key, value) in object {
                        if CourseCode::is_code(&key) {
                            vec.push(ExclusionCondition::Course(
                                CourseCode::from_str(&key).unwrap(),
                            ));
                        } else if ProgramCode::is_code(&key) {
                            vec.push(ExclusionCondition::Program(
                                ProgramCode::from_str(&key).unwrap(),
                            ));
                        } else {
                            vec.push(ExclusionCondition::PlainText(
                                value
                                    .as_str()
                                    .unwrap_or("Please check handbook for further detail")
                                    .to_string(),
                            ));
                        }
                    }
                }
                (string, vec)
            })
            .collect::<HashMap<String, Vec<ExclusionCondition>>>()
    }

    pub fn get_course(&self, course_code: &CourseCode) -> Result<&Course, String> {
        if !course_code.is_specific_course() {
            return Err(String::from(format!(
                "Expect a specific course code, rather than {}",
                &course_code
            )));
        }
        if !self.courses.contains_key(&course_code.to_string()) {
            return Err(String::from(format!(
                "{} cannot found in dataset",
                &course_code
            )));
        }
        Ok(self.courses.get(&course_code.to_string()).unwrap())
    }

    pub fn courses(&self) -> &HashMap<String, Course> {
        &self.courses
    }
    pub fn equivalent_courses(&self) -> &HashMap<String, Vec<String>> {
        &self.equivalent_courses
    }
    pub fn exclusion_courses(&self) -> &HashMap<String, Vec<ExclusionCondition>> {
        &self.exclusion_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_json() {
        let api = CourseManager::new(
            // /root/UNSW-HandBookX/backend/src/data/loader/src/program.rs
            // /root/UNSW-HandBookX/backend/src/data/loader/src/course.rs
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        assert_ne!(api.courses.len(), 0);
        assert_ne!(api.equivalent_courses.len(), 0);
        assert_ne!(api.exclusion_courses.len(), 0);
    }

    #[test]
    fn test_get_course() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP1511")).unwrap();
        assert_eq!(course.code(), String::from("COMP1511"));
        assert_eq!(course.uoc, 6);
        assert!(match course.campus {
            Campus::Sydney => true,
            _ => false,
        });
        assert_eq!(
            course.offering_terms,
            vec![
                OfferingTerm::Term1,
                OfferingTerm::Term2,
                OfferingTerm::Term3
            ]
        )
    }

    #[test]
    fn test_get_eqv() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        assert!(api
            .equivalent_courses
            .contains_key(&String::from("COMP1511")));
        let courses = api.equivalent_courses.get("COMP1511").unwrap();
        assert_eq!(
            courses,
            &vec![String::from("COMP1917"), String::from("DPST1091")]
        );
    }

    #[test]
    fn test_get_excl() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        assert!(api
            .exclusion_courses
            .contains_key(&String::from("COMP1511")));
        let courses = api.exclusion_courses.get("COMP1511").unwrap();
        assert_eq!(courses.len(), 2);
    }

    #[test]
    fn test_course_empty_requirement() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP1511")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3778").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_course() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP3153")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3778").unwrap(),
                &vec![String::from("MATH1081")],
                &None,
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_program() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP4961")).unwrap();
        assert!(!course
            .is_eligable(
                &ProgramCode::from_str("9999").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("4515").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3648").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_wam() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("BLDG4018")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("4522").unwrap(),
                &vec![],
                &Some(70),
                &api
            )
            .unwrap());
        assert!(!course
            .is_eligable(
                &ProgramCode::from_str("4522").unwrap(),
                &vec![],
                &Some(55),
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_uoc() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("BLDG4018")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("4522").unwrap(),
                &vec![String::from("DART1141")],
                &None,
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_binary() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP2511")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("COMP1531"), String::from("COMP2521")],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("COMP1531"), String::from("COMP1927")],
                &None,
                &api
            )
            .unwrap());
    }

    #[test]
    fn test_course_requriements_list() {
        let api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let course = api.get_course(&CourseCode::from("COMP4141")).unwrap();
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("MATH1081"), String::from("COMP2521")],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligable(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("MATH1081"), String::from("COMP1927")],
                &None,
                &api
            )
            .unwrap());
    }
}
