use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, BTreeSet, HashSet},
    mem::swap,
    ops::Deref,
};

use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelExtend, ParallelIterator,
};

use crate::{
    course::{self, CourseManager},
    program::{Course, Program, ProgramManager, Specialisation},
    utlis::{CourseCode, ProgramCode},
};

impl Program {
    pub fn list_courses(&self) -> Option<Vec<(&str, &Vec<Course>)>> {
        let course_components = self.course_component()?;
        // course_components
        let result: Vec<(&str, &Vec<Course>)> = course_components
            .par_iter()
            .map(|(key, value)| (value.title(), value.courses()))
            .collect();
        Some(result)
    }

    pub fn list_specialisations(&self) -> Option<[Vec<(&str, &Vec<String>)>; 3]> {
        let specialisation_component = self.specialisation_component()?;
        let major: Vec<(&str, &Vec<String>)> = if let Some(major) = specialisation_component.major()
        {
            major
                .par_iter()
                .map(|(key, value)| (key.as_ref(), value.specialiastions()))
                .collect()
        } else {
            Vec::new()
        };
        let minor: Vec<(&str, &Vec<String>)> = if let Some(minor) = specialisation_component.minor()
        {
            minor
                .par_iter()
                .map(|(key, value)| (key.as_ref(), value.specialiastions()))
                .collect()
        } else {
            Vec::new()
        };
        let honours: Vec<(&str, &Vec<String>)> =
            if let Some(honours) = specialisation_component.honours() {
                honours
                    .par_iter()
                    .map(|(key, value)| (key.as_ref(), value.specialiastions()))
                    .collect()
            } else {
                Vec::new()
            };
        Some([major, minor, honours])
    }
}

impl Specialisation {
    pub fn list_courses(&self) -> Vec<(&str, &Vec<Course>)> {
        self.course_component()
            .par_iter()
            .map(|(key, value)| (value.title(), value.courses()))
            .collect()
    }
}

impl ProgramManager {
    pub fn get_program_structure(
        &self,
        code: &ProgramCode,
        recusive: bool,
        specialiasation_codes: Option<&Vec<String>>,
    ) -> Result<Vec<(String, Vec<String>)>, String> {
        let program = self.get_program(code)?;
        let mut result: Vec<(String, Vec<String>)> = program
            .list_courses()
            .unwrap_or(Vec::new())
            .par_iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    value
                        .par_iter()
                        .map(|course| course.to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .collect();
        if recusive {
            if specialiasation_codes.is_none() {
                if let Some(specialiasations) = program.list_specialisations() {
                    for (components, name) in specialiasations
                        .iter()
                        .zip(vec!["Major", "Minor", "Honours"].iter())
                    {
                        for (direction, specs) in components {
                            for spec in *specs {
                                let spec = match self.get_specialiastion(&spec) {
                                    Ok(spec) => spec,
                                    Err(e) => {
                                        println!("Error(SPEC-CODE_1) : {}", e);
                                        continue;
                                    }
                                };
                                spec.list_courses().iter().for_each(
                                    |(spec_component_name, courses)| {
                                        let each: (String, Vec<String>) = (
                                            format!(
                                                "{} - {} - {}",
                                                name,
                                                spec.name(),
                                                spec_component_name
                                            ),
                                            courses
                                                .par_iter()
                                                .map(|course| course.to_string())
                                                .collect(),
                                        );
                                        result.push(each);
                                    },
                                );
                            }
                        }
                    }
                    Ok(result)
                } else {
                    Ok(result)
                }
            } else {
                for specialiasation_code in specialiasation_codes.unwrap() {
                    let spec = self.get_specialiastion(&specialiasation_code)?;
                    if !spec.programs().contains(&code) {
                        return Err(format!(
                            "Specialisation {} is not allowed for program {}",
                            specialiasation_code, code
                        ));
                    }
                    spec.list_courses()
                        .iter()
                        .for_each(|(spec_component_name, courses)| {
                            let each: (String, Vec<String>) = (
                                format!(
                                    "{} - {} - {}",
                                    spec.spec_type().to_string(),
                                    spec.name(),
                                    spec_component_name
                                ),
                                courses
                                    .par_iter()
                                    .map(|course| course.to_string())
                                    .collect(),
                            );
                            result.push(each);
                        });
                }
                Ok(result)
            }
        } else {
            if let Some(specialiasations) = program.list_specialisations() {
                for (components, name) in specialiasations
                    .iter()
                    .zip(vec!["Major", "Minor", "Honours"].iter())
                {
                    for (direction, specs) in components {
                        if specs.len() == 0 {
                            continue;
                        }
                        let each: (String, Vec<String>) = (
                            format!("{} - {}", name, direction,),
                            specs.par_iter().map(|spec| spec.to_string()).collect(),
                        );
                        result.push(each);
                    }
                }
                Ok(result)
            } else {
                Ok(result)
            }
        }
    }
    pub fn get_course_pool(&self, code: &ProgramCode) -> Result<SearchPool, String> {
        let program = self.get_program(code)?;
        let mut result: HashSet<&Course> = program
            .list_courses()
            .unwrap_or(Vec::new())
            .iter()
            .map(|(key, value)| value.iter())
            .flatten()
            .collect();
        if let Some(specialiasations) = program.list_specialisations() {
            for components in specialiasations {
                for (direction, specs) in components {
                    for spec in specs {
                        let spec = self.get_specialiastion(spec).unwrap();
                        spec.list_courses()
                            .iter()
                            .for_each(|(spec_component_name, courses)| {
                                result.extend(courses.iter());
                            });
                    }
                }
            }
        }
        let result = result
            .iter()
            .map(|course| course.to_course_codes())
            .flatten()
            .collect::<HashSet<CourseCode>>();

        Ok(SearchPool::new_from_set(result))
    }

    // pub fn
}
pub enum SearchPoolLevel {
    Hybrid,
    CourseCodeOnly,
    CoursePatternOnly,
}

pub struct SearchPool {
    course_code_pool: Option<HashSet<CourseCode>>,
    course_pattern_pool: Option<HashSet<CourseCode>>,
    pool_level: SearchPoolLevel,
}

impl SearchPool {
    pub fn new(
        course_code_pool: HashSet<CourseCode>,
        course_pattern_pool: HashSet<CourseCode>,
    ) -> Self {
        let course_code_pool_size = course_code_pool.len();
        let course_pattern_pool_size = course_pattern_pool.len();
        Self {
            course_code_pool: match course_code_pool_size {
                0 => None,
                _ => Some(course_code_pool),
            },
            course_pattern_pool: match course_pattern_pool_size {
                0 => None,
                _ => Some(course_pattern_pool),
            },
            pool_level: match (course_code_pool_size, course_pattern_pool_size) {
                (0, 0) => SearchPoolLevel::CourseCodeOnly,
                (_, 0) => SearchPoolLevel::CourseCodeOnly,
                (0, _) => SearchPoolLevel::CoursePatternOnly,
                (_, _) => SearchPoolLevel::Hybrid,
            },
        }
    }

    pub fn new_from_set(course_code_pool: HashSet<CourseCode>) -> Self {
        let course_pattern_pool = course_code_pool
            .par_iter()
            .filter(|course_code| course_code.is_pattern())
            .map(|course_code| course_code.clone())
            .collect::<HashSet<CourseCode>>();
        let course_code_pool = course_code_pool
            .into_par_iter()
            .filter(|course_code| !course_code.is_pattern())
            .collect::<HashSet<CourseCode>>();
        SearchPool::new(course_code_pool, course_pattern_pool)
    }

    pub fn set_search_level(&mut self, level: SearchPoolLevel) {
        self.pool_level = level;
    }

    pub fn adjust_pool_to_pattern(
        &mut self,
        num_of_match_school_code: u8,
        num_of_match_course_code: u8,
    ) {
        let course_pattern_pool: HashSet<CourseCode> =
            self.course_pattern_pool.take().unwrap_or(HashSet::new());
        let mut course_pattern_pool: HashSet<CourseCode> = course_pattern_pool
            .into_par_iter()
            .map(|mut pattern_code| {
                pattern_code.adjust_pattern(num_of_match_school_code, num_of_match_course_code);
                pattern_code
            })
            .collect();

        let course_code_pool: HashSet<CourseCode> =
            self.course_code_pool.take().unwrap_or(HashSet::new());
        let course_code_pool: HashSet<CourseCode> = course_code_pool
            .into_par_iter()
            .map(|mut course_code| {
                course_code.adjust_pattern(num_of_match_school_code, num_of_match_course_code);
                course_code
            })
            .collect();
        course_pattern_pool.par_extend(course_code_pool.into_par_iter());
        self.course_pattern_pool = Some(course_pattern_pool);
        self.pool_level = SearchPoolLevel::CoursePatternOnly;
    }

    pub fn pool_level(&self) -> &SearchPoolLevel {
        &self.pool_level
    }

    fn pool<'a, 'b>(&'a self, course_manager: &'b CourseManager) -> HashSet<&'b course::Course> {
        match self.pool_level {
            SearchPoolLevel::CourseCodeOnly => self.course_code_pool(course_manager),
            SearchPoolLevel::CoursePatternOnly => self.course_pattern_pool(course_manager),
            SearchPoolLevel::Hybrid => {
                let mut result = self.course_code_pool(course_manager);
                result.par_extend(self.course_pattern_pool(course_manager).into_par_iter());
                result
            }
        }
    }

    fn course_code_pool<'a, 'b>(
        &'a self,
        course_manager: &'b CourseManager,
    ) -> HashSet<&'b course::Course> {
        self.course_code_pool
            .as_ref()
            .unwrap_or(&HashSet::new())
            .par_iter()
            .map(|course_code| course_manager.get_course(course_code))
            .filter(|course| course.is_ok())
            .map(|course| course.unwrap())
            .collect()
    }

    fn course_pattern_pool<'a, 'b>(
        &'a self,
        course_manager: &'b CourseManager,
    ) -> HashSet<&'b course::Course> {
        course_manager
            .courses()
            .par_iter()
            .filter(|(course_code, course)| {
                self.course_pattern_pool
                    .as_ref()
                    .unwrap_or(&HashSet::new())
                    .par_iter()
                    .any(|pattern_code| pattern_code.is_match(course.course_code()))
            })
            .map(|(course_code, course)| course)
            .collect()
    }
}

impl CourseManager {
    pub fn list_eligable_courses<'a, 'b, 'c, 'd, 'e>(
        &'a self,
        search_pool: &'b SearchPool,
        program_code: &'c ProgramCode,
        taken_course: &'d Vec<String>,
        wam: &'e Option<u8>,
    ) -> Vec<&'a course::Course> {
        let pool: HashSet<&'a course::Course> = search_pool.pool(self);
        pool.into_par_iter()
            .filter(|course_code| {
                if let Ok(result) = course_code.is_eligable(program_code, taken_course, wam, self) {
                    return result;
                } else {
                    return false;
                }
            })
            .collect::<Vec<&'a course::Course>>()
    }

    pub fn list_courses_from_pool<'a, 'b>(
        &'a self,
        search_pool: &'b SearchPool,
    ) -> Vec<&'a course::Course> {
        search_pool
            .pool(self)
            .into_par_iter()
            .map(|course| course)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_list_courses_without_or() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let result = program.list_courses().unwrap();
        assert_eq!(result.len(), 4);
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Integrated First Year Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Prescribed Work Integrated Learning (WIL) Course"));
        assert!(result.iter().any(|(key, _)| key == &"Final Year Synthesis"));
        assert!(result.iter().any(|(key, _)| key == &"myBCom"));
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"myBCom")
                .unwrap()
                .1
                .len()
                == 3
        );
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Final Year Synthesis")
                .unwrap()
                .1
                .len()
                == 12
        );
    }
    #[test]
    fn test_program_list_courses_with_or() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3053").unwrap())
            .unwrap();
        let result = program.list_courses().unwrap();
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|(key, _)| key == &"Level 1 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 2 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Level 2 and Level 3 Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Level 3 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 4 Core Course"));
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 1 Core Courses")
                .unwrap()
                .1
                .len()
                == 6
        );
    }
    #[test]
    fn test_program_list_courses_with_any_course_pattern() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3053").unwrap())
            .unwrap();
        let result = program.list_courses().unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Level 2 and Level 3 Electives"));
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 2 and Level 3 Electives")
                .unwrap()
                .1
                .len()
                == 42
        );
    }

    #[test]
    fn test_program_list_courses_when_no_course_component() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3707").unwrap())
            .unwrap();
        let result = program.list_courses();
        assert!(result.is_none());
    }

    #[test]
    fn test_list_specialisations_with_major_and_minor() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3502").unwrap())
            .unwrap();
        let result = program.list_specialisations().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1); // has a major
        assert_eq!(result[1].len(), 1); // has a minor
        assert_eq!(result[2].len(), 0); // does not have honours
        assert!(result[0].iter().any(|(key, _)| key == &"Commerce"));
        assert!(
            result[0]
                .iter()
                .find(|(key, _)| key == &"Commerce")
                .unwrap()
                .1
                .len()
                == 12
        );
        assert!(result[1].iter().any(|(key, _)| key == &"Commerce"));
        assert!(
            result[1]
                .iter()
                .find(|(key, _)| key == &"Commerce")
                .unwrap()
                .1
                .len()
                == 24
        );
    }
    #[test]
    fn test_list_specialisations_with_honours() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3707").unwrap())
            .unwrap();
        let result = program.list_specialisations().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 0); // does not have a major
        assert_eq!(result[1].len(), 1); // has a minor
        assert_eq!(result[2].len(), 1); // has honours
        assert!(result[2]
            .iter()
            .any(|(key, _)| key == &"Engineering (Honours)"));
        assert_eq!(
            result[2]
                .iter()
                .find(|(key, _)| key == &"Engineering (Honours)")
                .unwrap()
                .1
                .len(),
            20
        );
        assert!(result[1]
            .iter()
            .any(|(key, _)| key == &"Engineering (Honours)"));
        assert_eq!(
            result[1]
                .iter()
                .find(|(key, _)| key == &"Engineering (Honours)")
                .unwrap()
                .1
                .len(),
            1
        );
    }

    #[test]
    fn test_list_specialisation_dual_degrees() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let result = program.list_specialisations().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2); // has two major
        assert_eq!(result[1].len(), 1); // has a minor
        assert!(result[0].iter().any(|(key, _)| key == &"Computer Science"));
        assert!(
            result[0]
                .iter()
                .find(|(key, _)| key == &"Computer Science")
                .unwrap()
                .1
                .len()
                == 8
        );
        assert!(result[0].iter().any(|(key, _)| key == &"Commerce"));
        assert!(
            result[0]
                .iter()
                .find(|(key, _)| key == &"Commerce")
                .unwrap()
                .1
                .len()
                == 12
        );
        assert!(result[1].iter().any(|(key, _)| key == &"Commerce"));
        assert!(
            result[1]
                .iter()
                .find(|(key, _)| key == &"Commerce")
                .unwrap()
                .1
                .len()
                == 14
        );
    }
    #[test]
    fn test_list_specialisation_dual_degree_honours() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3785").unwrap())
            .unwrap();
        let result = program.list_specialisations().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1); // has a major
        assert_eq!(result[1].len(), 0); // has a minor
        assert_eq!(result[2].len(), 1); // has a honours
        assert!(result[2]
            .iter()
            .any(|(key, _)| key == &"Engineering (Honours)"));
        assert!(
            result[2]
                .iter()
                .find(|(key, _)| key == &"Engineering (Honours)")
                .unwrap()
                .1
                .len()
                == 17
        );
        assert!(result[0].iter().any(|(key, _)| key == &"Computer Science"));
        assert!(
            result[0]
                .iter()
                .find(|(key, _)| key == &"Computer Science")
                .unwrap()
                .1
                .len()
                == 8
        );
    }

    #[test]
    fn test_list_specialisation_no_spec() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3053").unwrap())
            .unwrap();
        let result = program.list_specialisations();
        assert!(result.is_none());
    }

    #[test]
    fn test_major_specialisation_list_courses() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let spec = api.get_specialiastion("FINSA1").unwrap();
        let result = spec.list_courses();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|(key, _)| key == &"Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Prescribed Electives"));
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Core Courses")
                .unwrap()
                .1
                .len()
                == 5
        );
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Prescribed Electives")
                .unwrap()
                .1
                .len()
                == 34
        );
    }

    #[test]
    fn test_minor_specialisation_list_courses() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let spec = api.get_specialiastion("FINSA2").unwrap();
        let result = spec.list_courses();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|(key, _)| key == &"Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Prescribed Electives"));
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Core Courses")
                .unwrap()
                .1
                .len()
                == 3
        );
        assert!(
            result
                .iter()
                .find(|(key, _)| key == &"Prescribed Electives")
                .unwrap()
                .1
                .len()
                == 23
        );
    }

    #[test]
    fn test_honours_specialisation_list_courses() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let spec = api.get_specialiastion("TELEAH").unwrap();
        let result = spec.list_courses();
        assert_eq!(result.len(), 6);
        assert!(result.iter().any(|(key, _)| key == &"Level 1 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 2 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 3 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 4 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Discipline (Depth) Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Breadth Electives"));
        println!(
            "{:?}",
            result
                .iter()
                .find(|(key, _)| key == &"Level 1 Core Courses")
                .unwrap()
                .1
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 1 Core Courses")
                .unwrap()
                .1
                .len(),
            8
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 2 Core Courses")
                .unwrap()
                .1
                .len(),
            6
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 3 Core Courses")
                .unwrap()
                .1
                .len(),
            8
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 4 Core Courses")
                .unwrap()
                .1
                .len(),
            5
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Discipline (Depth) Electives")
                .unwrap()
                .1
                .len(),
            23
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Breadth Electives")
                .unwrap()
                .1
                .len(),
            21
        );
    }

    #[test]
    fn test_get_program_structure_simple_course_component() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3502").unwrap(), false, None)
            .unwrap();
        assert_eq!(result.len(), 6);
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Prescribed Work Integrated Learning (WIL) Course"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Integrated First Year Courses"));
        assert!(result.iter().any(|(key, _)| key == &"myBCom"));
        assert!(result.iter().any(|(key, _)| key == &"Final Year Synthesis"));
        assert!(result.iter().any(|(key, _)| key == &"Major - Commerce"));
        assert!(result.iter().any(|(key, _)| key == &"Minor - Commerce"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Prescribed Work Integrated Learning (WIL) Course")
                .unwrap()
                .1
                .len(),
            9
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Integrated First Year Courses")
                .unwrap()
                .1
                .len(),
            8
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"myBCom")
                .unwrap()
                .1
                .len(),
            3
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Final Year Synthesis")
                .unwrap()
                .1
                .len(),
            12
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Major - Commerce")
                .unwrap()
                .1
                .len(),
            12
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Minor - Commerce")
                .unwrap()
                .1
                .len(),
            24
        );
    }

    #[test]
    fn test_get_program_structure_simple_course_component_with_or() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3053").unwrap(), false, None)
            .unwrap();
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|(key, _)| key == &"Level 1 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 2 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Level 2 and Level 3 Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Level 3 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 4 Core Course"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Level 1 Core Courses")
                .unwrap()
                .1
                .len(),
            6
        );
    }

    #[test]
    fn test_get_program_structure_simple_no_course_component() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result =
            api.get_program_structure(&ProgramCode::from_str("3707").unwrap(), false, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
    #[test]
    fn test_get_program_structure_simple_dual_degree_course_component() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3784").unwrap(), false, None)
            .unwrap();
        assert_eq!(result.len(), 7);
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Integrated First Year Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Prescribed Work Integrated Learning (WIL) Course"));
        assert!(result.iter().any(|(key, _)| key == &"Final Year Synthesis"));
        assert!(result.iter().any(|(key, _)| key == &"myBCom"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science"));
        assert!(result.iter().any(|(key, _)| key == &"Major - Commerce"));
        assert!(result.iter().any(|(key, _)| key == &"Minor - Commerce"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Integrated First Year Courses")
                .unwrap()
                .1
                .len(),
            8
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Prescribed Work Integrated Learning (WIL) Course")
                .unwrap()
                .1
                .len(),
            9
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Final Year Synthesis")
                .unwrap()
                .1
                .len(),
            12
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"myBCom")
                .unwrap()
                .1
                .len(),
            3
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Major - Computer Science")
                .unwrap()
                .1
                .len(),
            8
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Major - Commerce")
                .unwrap()
                .1
                .len(),
            12
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Minor - Commerce")
                .unwrap()
                .1
                .len(),
            14
        );
    }
    #[test]
    fn test_get_program_structure_complex_course_component() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3786").unwrap(), true, None)
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Prescribed Theory Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Law Elective Courses"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Law Elective Courses")
                .unwrap()
                .1
                .len(),
            140 - 14
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Core Courses")
                .unwrap()
                .1
                .len(),
            17
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Prescribed Theory Electives")
                .unwrap()
                .1
                .len(),
            4
        );
    }

    #[test]
    fn test_get_program_structure_complex_spec() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3786").unwrap(), true, None)
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Prescribed Theory Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Law Elective Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key
                == &"Major - Computer Science (Computer Networks) - Computing Electives"));
        assert!(result.iter().any(|(key, _)| key
            == &"Major - Computer Science (Computer Networks) - Discipline Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science (Computer Networks) - Core Courses"));

        assert!(result.iter().any(|(key, _)| key == &"Major - Computer Science (Embedded Systems) - Embedded Systems Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science (Embedded Systems) - Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key
                == &"Major - Computer Science (Embedded Systems) - Computing Elective"));

        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key
                    == &"Major - Computer Science (Computer Networks) - Computing Electives")
                .unwrap()
                .1
                .len(),
            9
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key
                    == &"Major - Computer Science (Computer Networks) - Discipline Electives")
                .unwrap()
                .1
                .len(),
            5
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key
                    == &"Major - Computer Science (Computer Networks) - Core Courses")
                .unwrap()
                .1
                .len(),
            12
        );

        assert_eq!(result.iter().find(|(key, _)| key == &"Major - Computer Science (Embedded Systems) - Embedded Systems Prescribed Electives").unwrap().1.len(), 7);
        assert_eq!(
            result
                .iter()
                .find(
                    |(key, _)| key == &"Major - Computer Science (Embedded Systems) - Core Courses"
                )
                .unwrap()
                .1
                .len(),
            12
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key
                    == &"Major - Computer Science (Embedded Systems) - Computing Elective")
                .unwrap()
                .1
                .len(),
            9
        );
    }

    #[test]
    fn test_get_program_structure_complex_dual_degree() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3784").unwrap(), true, None)
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key
                == &"Major - Computer Science (Computer Networks) - Computing Electives"));
        assert!(result.iter().any(|(key, _)| key
            == &"Major - Computer Science (Computer Networks) - Discipline Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science (Computer Networks) - Core Courses"));

        assert!(result.iter().any(|(key, _)| key == &"Major - Computer Science (Embedded Systems) - Embedded Systems Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science (Embedded Systems) - Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key
                == &"Major - Computer Science (Embedded Systems) - Computing Elective"));

        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Accounting - Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Accounting - CAANZ/CPA Accreditation Requirements"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Accounting - Tax Practitioners Board Requirements"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Accounting - Core Courses"));

        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Business Economics - Flexible Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Business Economics - Compulsory Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Business Economics - Prescribed Electives"));

        assert!(result
            .iter()
            .any(|(key, _)| key == &"Minor - Finance - Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Minor - Finance - Core Courses"));
    }

    #[test]
    fn test_get_program_structure_complex_no_spec() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3053").unwrap(), true, None)
            .unwrap();
        assert!(result.iter().any(|(key, _)| key == &"Level 1 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 2 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Level 2 and Level 3 Electives"));
        assert!(result.iter().any(|(key, _)| key == &"Level 3 Core Courses"));
        assert!(result.iter().any(|(key, _)| key == &"Level 4 Core Course"));
    }

    #[test]
    fn test_get_program_stucture_complex_spec_with_honours() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(&ProgramCode::from_str("3707").unwrap(), true, None)
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 2 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Discipline (Depth) Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 1 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 3 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Breadth Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 4 Core Courses"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Honours - Telecommunications - Level 1 Core Courses")
                .unwrap()
                .1
                .len(),
            8
        );
    }

    #[test]
    fn test_get_program_structure_complex_given_one_code() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(
                &ProgramCode::from_str("3707").unwrap(),
                true,
                Some(&vec!["TELEAH".to_string()]),
            )
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 2 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Discipline (Depth) Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 1 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 3 Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Breadth Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Honours - Telecommunications - Level 4 Core Courses"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Honours - Telecommunications - Level 1 Core Courses")
                .unwrap()
                .1
                .len(),
            8
        );
    }

    #[test]
    fn test_get_program_strycture_complex_given_mul_codes() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api
            .get_program_structure(
                &ProgramCode::from_str("3784").unwrap(),
                true,
                Some(&vec![
                    "COMPA1".to_string(),
                    "FINSA1".to_string(),
                    "ACCTA2".to_string(),
                ]),
            )
            .unwrap();
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science - Computing Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Computer Science - Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Finance - Core Courses"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Major - Finance - Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Minor - Accounting - Prescribed Electives"));
        assert!(result
            .iter()
            .any(|(key, _)| key == &"Minor - Accounting - Core Courses"));
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Major - Computer Science - Computing Electives")
                .unwrap()
                .1
                .len(),
            9
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Major - Computer Science - Core Courses")
                .unwrap()
                .1
                .len(),
            11
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Minor - Accounting - Prescribed Electives")
                .unwrap()
                .1
                .len(),
            11
        );
        assert_eq!(
            result
                .iter()
                .find(|(key, _)| key == &"Minor - Accounting - Core Courses")
                .unwrap()
                .1
                .len(),
            2
        );
    }

    #[test]
    fn test_get_program_structure_err_program_does_not_exist() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api.get_program_structure(&ProgramCode::from_str("0000").unwrap(), true, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_program_structure_err_spec_does_not_exist() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api.get_program_structure(
            &ProgramCode::from_str("3784").unwrap(),
            true,
            Some(&vec![
                "COMPA1".to_string(),
                "FINSA1".to_string(),
                "ACCTA2".to_string(),
                "XXXXXX".to_string(),
            ]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_get_program_structure_err_spec_are_not_avaiable_for_program() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api.get_program_structure(
            &ProgramCode::from_str("3784").unwrap(),
            true,
            Some(&vec![
                "COMPA1".to_string(),
                "FINSA1".to_string(),
                "ACCTA2".to_string(),
                "TELEAH".to_string(),
            ]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_get_program_structure_err_program_does_not_exit_but_spec_exist() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let result = api.get_program_structure(
            &ProgramCode::from_str("9999").unwrap(),
            true,
            Some(&vec![
                "COMPA1".to_string(),
                "FINSA1".to_string(),
                "ACCTA2".to_string(),
                "TELEAH".to_string(),
            ]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_get_course_pool() {
        let program_api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let course_api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let pool = program_api.get_course_pool(&ProgramCode::from_str("3784").unwrap());
        assert!(pool.is_ok());
        let pool = pool.unwrap();
        let courses = course_api.list_courses_from_pool(&pool);
        assert!(courses.len() > 0);
    }

    #[test]
    fn test_list_eligible_courses() {
        let program_api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let course_api = CourseManager::new(
            "/root/UNSW-HandBookX/backend/data/coursesProcessed.json",
            "/root/UNSW-HandBookX/backend/data/equivalents.json",
            "/root/UNSW-HandBookX/backend/data/exclusions.json",
        );
        let pool = program_api.get_course_pool(&ProgramCode::from_str("3784").unwrap());
        assert!(pool.is_ok());
        let pool = pool.unwrap();
        let courses = course_api.list_eligable_courses(
            &pool,
            &ProgramCode::from_str("3784").unwrap(),
            &vec![],
            &None,
        );
        assert!(courses.len() > 0);
    }
}
