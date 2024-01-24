use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    ops::Deref,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    course::CourseManager,
    program::{Course, Program, ProgramManager, Specialisation},
    utlis::{CourseCode, ProgramCode},
};

impl Program {
    pub fn list_courses(&self) -> Option<Vec<(&str, &Vec<Course>)>> {
        let course_components = self.course_component()?;
        // course_components
        let result: Vec<(&str, &Vec<Course>)> = course_components
            .iter()
            .map(|(key, value)| (value.title(), value.courses()))
            .collect();
        Some(result)
    }

    pub fn list_specialisations(&self) -> Option<[Vec<(&str, &Vec<String>)>; 3]> {
        let specialisation_component = self.specialisation_component()?;
        let major: Vec<(&str, &Vec<String>)> = if let Some(major) = specialisation_component.major()
        {
            major
                .iter()
                .map(|(key, value)| (key.as_ref(), value.specialiastions()))
                .collect()
        } else {
            Vec::new()
        };
        let minor: Vec<(&str, &Vec<String>)> = if let Some(minor) = specialisation_component.minor()
        {
            minor
                .iter()
                .map(|(key, value)| (key.as_ref(), value.specialiastions()))
                .collect()
        } else {
            Vec::new()
        };
        let honours: Vec<(&str, &Vec<String>)> =
            if let Some(honours) = specialisation_component.honours() {
                honours
                    .iter()
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
            .iter()
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
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    value
                        .iter()
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
                                let spec = self.get_specialiastion(&spec)?;
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
                                                .iter()
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
                                courses.iter().map(|course| course.to_string()).collect(),
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
                    .zip(vec!["Major", "Minor", "Honours"].iter()) {
                    for (direction, specs) in components {
                        if specs.len() == 0 {
                            continue;
                        }
                        let each: (String, Vec<String>) = (
                            format!(
                                "{} - {}",
                                name,
                                direction,
                            ),
                            specs.iter().map(|spec| spec.to_string()).collect(),
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
    pub fn list_course_pool(&self, code: &ProgramCode) -> Result<Vec<CourseCode>, String> {
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
        Ok(result.into_iter().collect())
    }

    // pub fn
}

impl CourseManager {
    fn list_eligable_courses<'a, 'b, 'c, 'd, 'e>(
        &'a self,
        pool: &'b Vec<CourseCode>,
        program_code: &'c ProgramCode,
        taken_course: &'d Vec<String>,
        wam: &'e Option<u8>,
    ) -> Vec<&'b CourseCode> {
        pool.par_iter()
            .filter(|course_code| {
                let course = self.get_course(course_code);
                if course.is_err() {
                    return false;
                }
                if let Ok(result) =
                    course
                        .unwrap()
                        .is_eligable(program_code, taken_course, wam, self)
                {
                    return result;
                } else {
                    return false;
                }
            })
            .collect::<Vec<&CourseCode>>()
    }
}
