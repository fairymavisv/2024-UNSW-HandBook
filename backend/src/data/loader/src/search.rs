use std::{collections::BTreeMap, ops::Deref};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{program::{Program, Course, Specialisation, ProgramManager}, utlis::ProgramCode};



impl Program {
    pub fn list_courses(&self) -> Option<Vec<(&str, &Vec<Course>)>> {
        let course_components = self.course_component()?;
        // course_components
        let result: Vec<(&str, &Vec<Course>)> = course_components.iter().map(|(key, value)| {
            (value.title(), value.courses())
        }).collect();
        Some(result)
    }

    pub fn list_specialisations(&self) -> Option<[Vec<(&str, &Vec<String>)>; 3]> {
        let specialisation_component = self.specialisation_component()?;
        let major: Vec<(&str, &Vec<String>)> = if let Some(major) = specialisation_component.major() {
            major.iter().map(|(key, value)| {
                (key.as_ref(), value.specialiastions())
            }).collect()
        } else {
            Vec::new()
        };
        let minor: Vec<(&str, &Vec<String>)> = if let Some(minor) = specialisation_component.minor() {
            minor.iter().map(|(key, value)| {
                (key.as_ref(), value.specialiastions())
            }).collect()
        } else {
            Vec::new()
        };
        let honours: Vec<(&str, &Vec<String>)> = if let Some(honours) = specialisation_component.honours() {
            honours.iter().map(|(key, value)| {
                (key.as_ref(), value.specialiastions())
            }).collect()
        } else {
            Vec::new()
        };        
        Some([major, minor, honours])
    }
}

impl Specialisation {
    pub fn list_courses(&self) -> Vec<(&str, &Vec<Course>)> {
        self.course_component().iter().map(|(key, value)| {
            (value.title(), value.courses())
        }).collect()
    }
}

impl ProgramManager {
    pub fn get_program_structure(&self, code: &ProgramCode, recusive: bool, specialiasation_codes: Option<&Vec<String>>) -> Result<Vec<(String, Vec<String>)>, String> {
        let program = self.get_program(code)?;
        let mut result: Vec<(String, Vec<String>)> = program.list_courses().unwrap_or(Vec::new()).iter().map(|(key, value)| (key.to_string(), value.iter().map(|course| course.to_string()).collect::<Vec<String>>())).collect();
        if recusive {
            if specialiasation_codes.is_none() {
                if let Some(specialiasations) = program.list_specialisations() {
                    for (components, name) in  specialiasations.iter().zip(vec!["Major", "Minor", "Honours"].iter()) {
                        for (direction, specs) in components {
                            for spec in *specs {
                                let spec = self.get_specialiastion(&spec)?;
                                spec.list_courses().iter().for_each(|(spec_component_name, courses)| {
                                    let each: (String, Vec<String>) = (format!("{} - {}({}) - {}", name, direction, spec.name(), spec_component_name), courses.iter().map(|course| course.to_string()).collect());
                                    result.push(each);

                                });
                            }
                        }
                     };
                    Ok(result)
                } else {
                    Ok(result)
    
                }
            } else {
                // Assume given specialiasation_codes are allowed for given program
                // TODO varify the specialiasation_codes are valied
                for specialiasation_code in specialiasation_codes.unwrap() {
                    let spec = self.get_specialiastion(&specialiasation_code)?;
                    spec.list_courses().iter().for_each(|(spec_component_name, courses)| {
                        let each: (String, Vec<String>) = (format!("{} - {}", spec.name(), spec_component_name), courses.iter().map(|course| course.to_string()).collect());
                        result.push(each);
                        
                    });
                }
                Ok(result)


            }
            // program
        } else {
            Ok(result)
        }
    }
}