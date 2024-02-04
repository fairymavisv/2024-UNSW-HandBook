/// This module contains the implementation of the `Course` struct and the `CourseManager` struct.
/// The `Course` struct represents a course with its properties such as title, code, UOC (Units of Credit), description, level, study level, offering terms, campus, and requirements.
/// The `CourseManager` struct manages a collection of courses, equivalent courses, and exclusion courses.
/// It provides methods to parse course data from JSON files, retrieve courses by code, and access the course collection.
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
use std::hash::Hash;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Formatter};


#[derive(Serialize, Deserialize)]
/// Represents a list of courses in JSON format.
struct JSONCourseList {
    pub courses: HashMap<String, JSONCourse>,
}

/// Represents a list of JSON courses.
impl JSONCourseList {
    /// Creates a new `JSONCourseList` with the given courses.
    ///
    /// # Arguments
    ///
    /// * `courses` - A `HashMap` containing the courses, where the key is the course code and the value is the JSON representation of the course.
    ///
    /// # Returns
    ///
    /// A new `JSONCourseList` instance.
    pub fn new(courses: HashMap<String, JSONCourse>) -> Self {
        JSONCourseList { courses }
    }
}
#[derive(Serialize, Deserialize)]
/// Represents a course in JSON format.
struct JSONCourse {
    /// The title of the course.
    pub title: String,
    /// The code of the course.
    pub code: String,
    /// The number of units of credit (UOC) for the course.
    pub UOC: u8,
    /// The level of the course.
    pub level: u8,
    /// The description of the course.
    pub description: String,
    /// The study level of the course.
    pub study_level: String,
    /// The school offering the course.
    pub school: String,
    /// The faculty offering the course.
    pub faculty: String,
    /// The campus where the course is offered.
    pub campus: String,
    /// The equivalent courses for this course.
    pub equivalents: HashMap<String, Value>,
    /// The excluded courses for this course.
    pub exclusions: HashMap<String, Value>,
    /// The terms in which the course is offered.
    pub terms: Vec<String>,
    /// Indicates if the course is a general education course.
    pub gen_ed: bool,
    /// The raw requirements for the course.
    pub raw_requirements: String,
    /// Indicates if the course is a multi-term course.
    pub is_multiterm: bool,
}

/// Representation the exculsion condition for a course.
/// It can be a course code, a program code, or a plain text.
pub enum ExclusionCondition {
    Course(CourseCode),
    Program(ProgramCode),
    PlainText(String),
}


#[derive(Clone)]
/// Represents a course in the university handbook.
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
    /// Represents a course in the university.
    ///
    /// A course has various attributes such as title, code, UOC (units of credit), description,
    /// level, study level, offering terms, campus, and requirements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use crate::data::loader::course::{Course, CourseCode, StudyLevel, OfferingTerm, Campus, Requirements};
    ///
    /// // Create a new course
    /// let course = Course::new(
    ///     String::from("Introduction to Programming"),
    ///     String::from("COMP1511"),
    ///     6,
    ///     String::from("An introduction to programming concepts and techniques."),
    ///     1,
    ///     StudyLevel::Undergraduate,
    ///     vec![OfferingTerm::Term1, OfferingTerm::Term2],
    ///     Campus::Kensington,
    ///     Some(Arc::new(Requirements::new(""))),
    /// );
    ///
    /// // Check if the course is available in a specific offering term
    /// let term = OfferingTerm::Term1;
    /// assert!(course.available_at_term(&term));
    ///
    /// // Get the course title
    /// assert_eq!(course.title(), "Introduction to Programming");
    ///
    /// // Get the course code
    /// assert_eq!(course.code(), "COMP1511");
    ///
    /// // Get the course UOC
    /// assert_eq!(course.uoc(), 6);
    ///
    /// // Get the course description
    /// assert_eq!(course.description(), "An introduction to programming concepts and techniques.");
    ///
    /// // Get the course level
    /// assert_eq!(*course.level(), 1);
    ///
    /// // Get the course study level
    /// assert_eq!(course.study_level(), &StudyLevel::Undergraduate);
    ///
    /// // Get the offering terms for the course
    /// assert_eq!(course.offering_terms(), &[OfferingTerm::Term1, OfferingTerm::Term2]);
    ///
    /// // Get the campus where the course is offered
    /// assert_eq!(course.campus(), &Campus::Kensington);
    ///
    /// // Get the requirements for the course
    /// assert_eq!(course.requirements(), Some(Arc::new(Requirements::new())));
    /// ```
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

    /// Checks if the course is available in the given offering term.
    /// 
    /// # Arguments
    /// 
    /// * `offering_terms` - The offering term to check.
    /// 
    /// # Returns
    /// 
    /// `true` if the course is available in the given offering term, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::{Course, CourseCode, StudyLevel, OfferingTerm, Campus, Requirements};
    /// 
    /// // Create a new course
    /// let course = Course::new(
    ///     String::from("Introduction to Programming"),
    ///     String::from("COMP1511"),
    ///     6,
    ///     String::from("An introduction to programming concepts and techniques."),
    ///     1,
    ///     StudyLevel::Undergraduate,
    ///     vec![OfferingTerm::Term1, OfferingTerm::Term2],
    ///     Campus::Kensington,
    ///     Some(Arc::new(Requirements::new(""))),
    /// );
    /// 
    /// // Check if the course is available in a specific offering term
    /// let term = OfferingTerm::Term1;
    /// assert!(course.available_at_term(&term));
    /// 
    /// let term = OfferingTerm::Term2;
    /// assert!(course.available_at_term(&term));
    /// 
    /// let term = OfferingTerm::Term3;
    /// assert!(!course.available_at_term(&term));
    /// 
    /// ```
    pub fn available_at_term(&self, offering_terms: &OfferingTerm) -> bool {
        self.offering_terms.contains(offering_terms)
    }

    /// Checks if the course is eligible for the given program code, taken courses, WAM.
    /// 
    /// # Arguments
    /// 
    /// * `program_code` - The program code of users.
    /// * `taken_course` - The list of taken courses.
    /// * `wam` - The WAM.
    /// * `course_manager` - The course manager.
    /// 
    /// # Returns
    /// 
    /// `true` if the course is eligible for the given program code, taken courses, and WAM, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::{Course, CourseCode, StudyLevel, OfferingTerm, Campus, Requirements, CourseManager, ProgramCode};
    /// 
    /// // Create a new course
    /// let course = Course::new(
    ///     String::from("Introduction to Programming"),
    ///     String::from("COMP1511"),
    ///     6,
    ///     String::from("An introduction to programming concepts and techniques."),
    ///     1,
    ///     StudyLevel::Undergraduate,
    ///     vec![OfferingTerm::Term1, OfferingTerm::Term2],
    ///     Campus::Kensington,
    ///     Some(Arc::new(Requirements::new("Pre-requisite: Wam of 85"))),
    /// );
    /// 
    /// // Create a new course manager
    /// let course_manager = CourseManager::empty();
    /// 
    /// // Check if the course is eligible for the given program code, taken courses, and WAM
    /// let program_code = ProgramCode::from_str("3778").unwrap();
    /// let taken_course = vec![];
    /// let wam = Some(90);
    /// 
    /// assert!(course.is_eligible(&program_code, &taken_course, &wam, &course_manager).unwrap());
    /// 
    /// let wam = Some(80);
    /// assert!(!course.is_eligible(&program_code, &taken_course, &wam, &course_manager).unwrap());
    /// 
    /// ```
    /// 
    pub fn is_eligible(
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

    /// Gets the title of the course.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Gets the course code of the course.
    pub fn code(&self) -> String {
        self.code.to_string()
    }

    /// Gets the course code of the course.
    pub fn course_code(&self) -> &CourseCode {
        &self.code
    }

    /// Gets the UOC of the course.
    pub fn uoc(&self) -> u8 {
        self.uoc
    }

    /// Gets the description of the course.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Gets the level of the course.
    pub fn level(&self) -> &u8 {
        &self.level
    }

    /// Gets the study level of the course.
    pub fn study_level(&self) -> &StudyLevel {
        &self.study_level
    }

    /// Gets the offering terms of the course.
    pub fn offering_terms(&self) -> &Vec<OfferingTerm> {
        &self.offering_terms
    }

    /// Gets the campus of the course.
    pub fn campus(&self) -> &Campus {
        &self.campus
    }

    /// Gets the requirements of the course.
    pub fn requirements(&self) -> Option<Arc<Requirements>> {
        self.requirements.as_ref().map(|x| x.clone())
    }
}

impl Hash for Course {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl Eq for Course {
    fn assert_receiver_is_total_eq(&self) {
        //
    }
}
impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Display for Course {
    /// Formats the course as a string.
    /// 
    /// # Returns
    /// 
    /// A string representation of the course.
    /// 
    /// e.g. COMP1511 - Introduction to Programming
    /// 
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.code, self.title)
    }
}
/// A struct that represents a course manager.
/// It contains a collection of courses, equivalent courses, and exclusion courses.
/// It provides methods to parse course data from JSON files, retrieve courses by code, and access the course collection.
/// 
pub struct CourseManager {
    courses: HashMap<String, Course>,
    equivalent_courses: HashMap<String, Vec<String>>,
    exclusion_courses: HashMap<String, Vec<ExclusionCondition>>,
}

impl CourseManager {

    /// Creates a new course manager with empty course, equivalent course, and exclusion course collections.
    /// 
    /// # Returns
    /// 
    /// A new `CourseManager` instance.
    pub fn empty() -> CourseManager {
        CourseManager {
            courses: HashMap::new(),
            equivalent_courses: HashMap::new(),
            exclusion_courses: HashMap::new(),
        }
    }

    /// Loads the course, equivalent course, and exclusion course data from the given JSON files.
    /// 
    /// # Arguments
    /// 
    /// * `courses_path` - The path to the JSON file containing the course data.
    /// * `equivalent_path` - The path to the JSON file containing the equivalent course data.
    /// * `exclusion_path` - The path to the JSON file containing the exclusion course data.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::CourseManager;
    ///     
    /// // Create a new course manager
    /// let mut course_manager = CourseManager::empty();
    /// 
    /// // Load the course, equivalent course, and exclusion course data from the given JSON files
    /// course_manager.load("courses.json", "equivalents.json", "exclusions.json");
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if the JSON files cannot be read.
    /// 
    /// # Notice
    /// This function will ignore UNSW Global's courses and Canberra campus's courses.
    pub fn load(&mut self, courses_path: &str, equivalent_path: &str, exclusion_path: &str) {
        self.courses = CourseManager::parse_from_courses_json(courses_path);
        self.equivalent_courses = CourseManager::parse_from_equivalent_json(equivalent_path);
        self.exclusion_courses = CourseManager::parse_from_exclusion_json(exclusion_path);
    }

    /// Creates a new course manager with the course, equivalent course, and exclusion course data from the given JSON files.
    ///     
    /// 
    /// # Arguments
    /// 
    /// * `courses_path` - The path to the JSON file containing the course data.
    /// * `equivalent_path` - The path to the JSON file containing the equivalent course data.
    /// * `exclusion_path` - The path to the JSON file containing the exclusion course data.
    /// 
    /// # Returns
    /// 
    /// A new `CourseManager` instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::CourseManager;
    /// 
    /// // Create a new course manager
    /// let course_manager = CourseManager::new("courses.json", "equivalents.json", "exclusions.json");
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if the JSON files cannot be read.
    /// 
    /// # Notice
    /// This function will ignore UNSW Global's courses and Canberra campus's courses.
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

    /// Parses the course data from the given JSON file.
    /// 
    /// # Arguments
    /// 
    /// * `json_path` - The path to the JSON file containing the course data.
    /// 
    /// # Returns
    /// 
    /// A `HashMap` containing the courses, where the key is the course code and the value is the course.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::CourseManager;
    /// 
    /// // Parse the course data from the given JSON file
    /// 
    /// let courses = CourseManager::parse_from_courses_json("courses.json");
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// - Panics if the JSON file cannot be read.
    /// 
    /// - Panics if `StudyLevel` is not expected.
    /// 
    /// - Panics if `Campus` is not expected.
    /// 
    /// # Notice
    /// 
    /// `Requirement` is none if there is an error in parsing the requirement.
    /// 
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

    /// Parses the equivalent course data from the given JSON file.
    /// 
    /// # Arguments
    /// 
    /// * `json_path` - The path to the JSON file containing the equivalent course data.
    /// 
    /// # Returns
    /// 
    /// A `HashMap` containing the equivalent courses, where the key is the course code and the value is a list of equivalent course codes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::CourseManager;
    /// 
    /// // Parse the equivalent course data from the given JSON file
    /// 
    /// let equivalent_courses = CourseManager::parse_from_equivalent_json("equivalents.json");
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// - Panics if the JSON file cannot be read.
    /// 
    /// - Panics if the JSON parsing fails.
    /// 
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

    /// Parses the exclusion course data from the given JSON file.
    /// 
    /// # Arguments
    /// 
    /// * `json_path` - The path to the JSON file containing the exclusion course data.
    /// 
    /// # Returns
    /// 
    /// A `HashMap` containing the exclusion courses, where the key is the course code and the value is a list of exclusion conditions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::CourseManager;
    /// 
    /// // Parse the exclusion course data from the given JSON file
    /// 
    /// let exclusion_courses = CourseManager::parse_from_exclusion_json("exclusions.json");
    /// 
    /// ```
    /// 
    /// # Panics
    /// 
    /// - Panics if the JSON file cannot be read.
    /// 
    /// - Panics if the JSON parsing fails.
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

    /// Gets the course with the given course code.
    /// 
    /// # Arguments
    /// 
    /// * `course_code` - The course code of the course to retrieve. The course code must be a valid course code, rather than a course code pattern.
    /// 
    /// # Returns
    /// 
    /// The course with the given course code, or an error message if the course cannot be found.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::data::loader::course::{CourseManager, CourseCode};
    /// 
    /// // Create a new course manager
    /// let course_manager = CourseManager::new("courses.json", "equivalents.json", "exclusions.json");
    /// 
    /// // Get the course with the given course code
    /// let course = course_manager.get_course(&CourseCode::from("COMP1511")).unwrap();
    /// 
    /// ```
    /// 
    pub fn get_course(&self, course_code: &CourseCode) -> Result<&Course, String> {
        if !course_code.is_course_code() {
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

    /// Gets the collection of courses.
    /// Key is the course code
    /// Value is the course object
    pub fn courses(&self) -> &HashMap<String, Course> {
        &self.courses
    }
    
    /// Gets the collection of equivalent courses.
    /// Key is the course code
    /// Value is a list of equivalent course codes.
    pub fn equivalent_courses(&self) -> &HashMap<String, Vec<String>> {
        &self.equivalent_courses
    }

    /// Gets the collection of exclusion courses.
    /// Key is the course code
    /// Value is a list of exclusion conditions.
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
            .is_eligible(
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
            .is_eligible(
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
            .is_eligible(
                &ProgramCode::from_str("9999").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligible(
                &ProgramCode::from_str("4515").unwrap(),
                &vec![],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligible(
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
            .is_eligible(
                &ProgramCode::from_str("4522").unwrap(),
                &vec![],
                &Some(70),
                &api
            )
            .unwrap());
        assert!(!course
            .is_eligible(
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
            .is_eligible(
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
            .is_eligible(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("COMP1531"), String::from("COMP2521")],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligible(
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
            .is_eligible(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("MATH1081"), String::from("COMP2521")],
                &None,
                &api
            )
            .unwrap());
        assert!(course
            .is_eligible(
                &ProgramCode::from_str("3784").unwrap(),
                &vec![String::from("MATH1081"), String::from("COMP1927")],
                &None,
                &api
            )
            .unwrap());
    }
}
