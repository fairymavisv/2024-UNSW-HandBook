use crate::{utlis::{StudyLevel, Campus, OfferingTerm}, JSONCourse, JSONCourseList};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};
pub struct Course {
    title: String,
    code: String,
    uoc: u8,
    level: u8,
    study_level: StudyLevel,
    offering_terms: Vec<OfferingTerm>,
    campus: Campus,
} 
impl Course {
    pub fn new(title: String, code: String, uoc: u8, level: u8, study_level: StudyLevel, offering_terms: Vec<OfferingTerm>, campus: Campus) -> Course {
        Course {
            title,
            code,
            uoc,
            level,
            study_level,
            offering_terms,
            campus
        }
    }

    pub fn parse_from_courses_json(json_courses: JSONCourseList) -> Vec<Course> {
        let course_list: Vec<Course> = json_courses.courses.into_par_iter().map(|json_course| {
            let title = json_course.title;
            let code = json_course.code;
            let uoc = json_course.uoc;
            let level = json_course.level;
            let study_level = StudyLevel::from_str(&json_course.study_level).expect(format!("Unexpected study level: {}", json_course.study_level).as_str());
            let offering_terms = json_course.terms.iter().filter_map(|term| OfferingTerm::from_str(term)).collect::<Vec<OfferingTerm>>();
            let campus = Campus::from_str(&json_course.campus).expect(format!("Unexpected campus: {}", json_course.campus).as_str());
            Course::new(title, code, uoc, level, study_level, offering_terms, campus)
        }).collect();
        course_list
    }
}

