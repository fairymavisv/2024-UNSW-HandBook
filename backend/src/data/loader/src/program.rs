use std::{collections::HashMap, fmt::Display, fs, hash::Hash, process::Output};

use crate::{
    course,
    utlis::{CourseCode, ProgramCode, StudyLevel},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::{json, Value};
#[derive(Clone)]
pub struct Program {
    title: String,
    code: ProgramCode,
    uoc: u8,
    duration: u8,
    overview: String,
    structure_summary: String,
    course_components: Option<HashMap<String, CourseComponent>>,
    specialisation_component: Option<SpecialisationComponent>,
    rules: Vec<Rules>,
}

impl Program {
    fn new_from_json(json: &serde_json::Map<String, Value>) -> Self {
        let title = json.get("title").unwrap().as_str().unwrap().to_string();
        let code = ProgramCode::from_str(json.get("code").unwrap().as_str().unwrap()).unwrap();
        let uoc = json.get("UOC").unwrap().as_i64().unwrap() as u8;
        let duration = json.get("duration").unwrap().as_i64().unwrap() as u8;
        let overview = json
            .get("overview")
            .unwrap()
            .as_str()
            .unwrap_or("")
            .to_string();
        let structure_summary = json
            .get("structure_summary")
            .unwrap()
            .as_str()
            .unwrap_or("")
            .to_string();
        let json_components = json.get("components").unwrap().as_object().unwrap();

        let mut buffed_rules: Vec<Rules> = Vec::new();
        // TODO: build components from non-spec part
        let course_components = if let Some(non_spec) = json_components.get("non_spec_data") {
            let non_spec = non_spec.as_array().unwrap();
            let mut buffed_components: HashMap<String, CourseComponent> = HashMap::new();
            non_spec
                .iter()
                .map(|object| object.as_object().unwrap())
                .for_each(|object| {
                    let rules_type = object.get("type").expect(&title).as_str().unwrap();
                    match rules_type {
                        "prescribed_electives" | "core_courses" => {
                            let component: CourseComponent =
                                ProgramComponentBuilder::build(object).unwrap();
                            buffed_components.insert(component.title.clone(), component);
                        }
                        "info_rule" | "limit_rule" => {
                            let rule: Rules = ProgramComponentBuilder::build(object).unwrap();
                            buffed_rules.push(rule);
                        }
                        _ => (),
                    }
                });
            if buffed_components.len() == 0 {
                None
            } else {
                Some(buffed_components)
            }
        } else {
            None
        };
        // TODO: build components from spec part
        let specialisation_component = if let Some(spec_data) = json_components.get("spec_data") {
            let spec = spec_data.as_object().unwrap();
            ProgramComponentBuilder::build(spec)
        } else {
            None
        };

        Self {
            title: title,
            code: code,
            uoc: uoc,
            duration: duration,
            overview: overview,
            structure_summary: structure_summary,
            course_components: course_components,
            specialisation_component: specialisation_component,
            rules: buffed_rules,
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

    pub fn duration(&self) -> u8 {
        self.duration
    }

    pub fn overview(&self) -> &str {
        &self.overview
    }

    pub fn structure_summary(&self) -> &str {
        &self.overview
    }

    pub fn course_component(&self) -> Option<&HashMap<String, CourseComponent>> {
        self.course_components.as_ref()
    }

    pub fn specialisation_component(&self) -> Option<&SpecialisationComponent> {
        self.specialisation_component.as_ref()
    }

    pub fn rules(&self) -> &Vec<Rules> {
        &self.rules
    }
}

#[derive(Clone)]
pub enum SpecialisationType {
    Major,
    Minor,
    Honours,
}

impl Display for SpecialisationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialisationType::Major => write!(f, "Major"),
            SpecialisationType::Minor => write!(f, "Minor"),
            SpecialisationType::Honours => write!(f, "Honours"),
        }
    }
}

#[derive(Clone)]
pub struct Specialisation {
    name: String,
    spec_type: SpecialisationType,
    uoc: u8,
    code: String,
    course_components: HashMap<String, CourseComponent>,
    constraints: Option<Vec<Constraints>>,
    programs: Vec<ProgramCode>,
}

impl Specialisation {
    fn new_from_json(json: &serde_json::Map<String, Value>) -> Self {
        let name = json.get("name").unwrap().as_str().unwrap().to_string();
        let code = json.get("code").unwrap().as_str().unwrap().to_string();
        let uoc = json.get("UOC").unwrap().as_i64().unwrap() as u8;
        let curriculum = json.get("curriculum").unwrap().as_array().unwrap();
        let course_components: HashMap<String, CourseComponent> = curriculum
            .iter()
            .map(|object| {
                let json = object.as_object().unwrap();
                let course_component: CourseComponent =
                    ProgramComponentBuilder::build(json).unwrap();
                (course_component.title.clone(), course_component)
            })
            .collect();
        let constraints = json.get("course_constraints").unwrap().as_array().unwrap();
        let constraints: Vec<Constraints> = constraints
            .iter()
            .map(|object| {
                let constraint = object.as_object().unwrap();
                let title = constraint
                    .get("title")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                let description = constraint
                    .get("description")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                Constraints::new(title, description)
            })
            .collect();
        let spec_type = match json.get("type").unwrap().as_str().unwrap() {
            "major" => SpecialisationType::Major,
            "minor" => SpecialisationType::Minor,
            "honours" => SpecialisationType::Honours,
            _ => panic!("Invalid specialisation type"),
        };
        Self {
            name,
            spec_type,
            uoc,
            code,
            course_components,
            constraints: if constraints.len() == 0 {
                None
            } else {
                Some(constraints)
            },
            programs: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn spec_type(&self) -> &SpecialisationType {
        &self.spec_type
    }

    pub fn uoc(&self) -> u8 {
        self.uoc
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn course_component(&self) -> &HashMap<String, CourseComponent> {
        &self.course_components
    }

    pub fn constraints(&self) -> Option<&Vec<Constraints>> {
        self.constraints.as_ref()
    }

    pub fn programs(&self) -> &Vec<ProgramCode> {
        &self.programs
    }
}

#[derive(Clone)]
pub struct Constraints {
    title: String,
    description: String,
}
impl Constraints {
    fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}
#[derive(Clone, Debug)]
pub enum Course {
    Course(CourseCode),
    Alternative(AlternativeCourse),
    Text(String),
}
impl Course {
    fn new(course_code: &str) -> Self {
        if let Some(ac) = AlternativeCourse::from_str(course_code) {
            Course::Alternative(ac)
        } else if let Some(c) = CourseCode::parse(course_code.trim()) {
            Course::Course(c)
        } else {
            Course::Text(course_code.to_string())
        }
    }

    pub fn to_course_codes(&self) -> Vec<CourseCode> {
        match self {
            Course::Course(c) => vec![c.clone()],
            Course::Alternative(ac) => ac.courses.clone(),
            Course::Text(_) => Vec::new(),
        }
    }
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Course::Alternative(al) => al.fmt(f),
            Course::Course(c) => c.fmt(f),
            Course::Text(t) => t.fmt(f),
        }
    }
}

impl Hash for Course {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Eq for Course {
    fn assert_receiver_is_total_eq(&self) {
        // do nothing
    }
}

#[derive(Clone, Debug)]
pub struct AlternativeCourse {
    courses: Vec<CourseCode>,
}
impl Display for AlternativeCourse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.courses
                .iter()
                .map(|code| code.to_string())
                .collect::<Vec<String>>()
                .join(" or ")
        )
    }
}
impl AlternativeCourse {
    fn new(courses: Vec<CourseCode>) -> Self {
        AlternativeCourse { courses }
    }

    fn from_str(courses: &str) -> Option<Self> {
        let mut buf = Vec::new();
        if !courses.contains("or") {
            return None;
        }
        for course in courses.split("or") {
            if let Some(course_code) = CourseCode::parse(course.trim()) {
                buf.push(course_code);
            } else {
                return None;
            }
        }
        Some(Self { courses: buf })
    }
}

#[derive(Clone)]
pub struct CourseComponent {
    title: String,
    courses: Vec<Course>,
    uoc: u8,
    note: String,
}

impl CourseComponent {
    fn new(title: String, courses: Vec<Course>, uoc: u8, note: String) -> Self {
        Self {
            title,
            courses,
            uoc,
            note,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn courses(&self) -> &Vec<Course> {
        &self.courses
    }

    pub fn uoc(&self) -> u8 {
        self.uoc
    }

    pub fn note(&self) -> &str {
        &self.note
    }
}

#[derive(Clone)]
pub struct SpecialisationView {
    specialisations: Vec<String>,
    notes: String,
    is_optional: bool,
}

impl SpecialisationView {
    fn new(specialisations: Vec<String>, notes: String, is_optional: bool) -> Self {
        Self {
            specialisations,
            notes,
            is_optional,
        }
    }

    pub fn specialiastions(&self) -> &Vec<String> {
        &self.specialisations
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn is_optional(&self) -> bool {
        self.is_optional
    }
}

#[derive(Clone)]
pub struct SpecialisationComponent {
    major: Option<HashMap<String, SpecialisationView>>,
    minor: Option<HashMap<String, SpecialisationView>>,
    honours: Option<HashMap<String, SpecialisationView>>,
}

impl SpecialisationComponent {
    fn new(
        major: Option<HashMap<String, SpecialisationView>>,
        minor: Option<HashMap<String, SpecialisationView>>,
        honours: Option<HashMap<String, SpecialisationView>>,
    ) -> Self {
        Self {
            major,
            minor,
            honours,
        }
    }

    pub fn major(&self) -> Option<&HashMap<String, SpecialisationView>> {
        self.major.as_ref()
    }

    pub fn minor(&self) -> Option<&HashMap<String, SpecialisationView>> {
        self.minor.as_ref()
    }

    pub fn honours(&self) -> Option<&HashMap<String, SpecialisationView>> {
        self.honours.as_ref()
    }
}

#[derive(Clone)]
pub enum Rules {
    Info(InfoRule),
    Limit(InfoRule),
}
impl Rules {
    pub fn title(&self) -> &str {
        match self {
            Rules::Info(i) => i.title(),
            Rules::Limit(l) => l.title(),
        }
    }

    pub fn body(&self) -> &str {
        match self {
            Rules::Info(i) => i.body(),
            Rules::Limit(l) => l.body(),
        }
    }
}

#[derive(Clone)]
pub struct InfoRule {
    title: String,
    body: String,
}

impl InfoRule {
    fn new(title: String, body: String) -> Self {
        Self { title, body }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

pub struct ProgramManager {
    programs: HashMap<String, Program>,
    specialiastions: HashMap<String, Specialisation>,
}

impl ProgramManager {
    pub fn new(program_json: &str, specialiastions: &str) -> Self {
        let mut manager = Self {
            programs: ProgramManager::parse_from_program_json(program_json),
            specialiastions: ProgramManager::parse_from_specialisation_json(specialiastions),
        };
        manager.mapping_program_into_specialisation();
        manager
    }

    fn mapping_program_into_specialisation(&mut self) {
        self.programs.iter().for_each(|(_, program)| {
            if let Some(specialisation_component) = program.specialisation_component.as_ref() {
                if let Some(major) = specialisation_component.major.as_ref() {
                    major.iter().for_each(|(major_name, major_view)| {
                        major_view.specialisations.iter().for_each(|spec_code| {
                            if let Some(specialisation) = self.specialiastions.get_mut(spec_code) {
                                specialisation.programs.push(program.code.clone());
                            }
                        });
                    });
                }
                if let Some(minor) = specialisation_component.minor.as_ref() {
                    minor.iter().for_each(|(minor_name, minor_view)| {
                        minor_view.specialisations.iter().for_each(|spec_code| {
                            if let Some(specialisation) = self.specialiastions.get_mut(spec_code) {
                                specialisation.programs.push(program.code.clone());
                            }
                        });
                    });
                }
                if let Some(honours) = specialisation_component.honours.as_ref() {
                    honours.iter().for_each(|(honours_name, honours_view)| {
                        honours_view.specialisations.iter().for_each(|spec_code| {
                            if let Some(specialisation) = self.specialiastions.get_mut(spec_code) {
                                specialisation.programs.push(program.code.clone());
                            }
                        });
                    });
                }
            }
        });
    }

    fn parse_from_program_json(json_path: &str) -> HashMap<String, Program> {
        let json = fs::read_to_string(json_path).expect("Unable to read program json file");
        let json_programs: HashMap<String, Value> = serde_json::from_str(&json).unwrap();
        json_programs
            .into_par_iter()
            .map(|(program_code, json_value)| {
                (
                    program_code,
                    Program::new_from_json(json_value.as_object().unwrap()),
                )
            })
            .collect()
    }

    fn parse_from_specialisation_json(json_path: &str) -> HashMap<String, Specialisation> {
        let json = fs::read_to_string(json_path).expect("Unable to read specialisation json file");
        let json_specialisation: HashMap<String, Value> = serde_json::from_str(&json).unwrap();
        json_specialisation
            .into_par_iter()
            .map(|(program_code, json_value)| {
                (
                    program_code,
                    Specialisation::new_from_json(json_value.as_object().unwrap()),
                )
            })
            .collect()
    }

    pub fn programs(&self) -> &HashMap<String, Program> {
        &self.programs
    }

    pub fn specialiastions(&self) -> &HashMap<String, Specialisation> {
        &self.specialiastions
    }

    pub fn get_program(&self, code: &ProgramCode) -> Result<&Program, String> {
        if let Some(course) = self.programs.get(&code.to_string()) {
            Ok(course)
        } else {
            Err(String::from(format!("{} cannot found in dataset", &code)))
        }
    }

    pub fn get_specialiastion(&self, code: &str) -> Result<&Specialisation, String> {
        if let Some(specialisation) = self.specialiastions.get(&code.to_string()) {
            Ok(specialisation)
        } else {
            Err(String::from(format!("{} cannot found in dataset", &code)))
        }
    }
}

pub struct ProgramComponentBuilder {}

pub trait Builder<Output> {
    fn build(json: &serde_json::Map<String, Value>) -> Option<Output>;
}

impl Builder<CourseComponent> for ProgramComponentBuilder {
    fn build(json: &serde_json::Map<String, Value>) -> Option<CourseComponent> {
        let courses: &serde_json::Map<String, Value> = json.get("courses")?.as_object()?;
        let mut courses_buf = Vec::new();
        courses.keys().into_iter().for_each(|each| {
            courses_buf.push(Course::new(&each));
        });
        Some(CourseComponent::new(
            json.get("title")?.as_str()?.to_string(),
            courses_buf,
            json.get("credits_to_complete")?.as_i64()? as u8,
            json.get("notes")?.as_str()?.to_string(),
        ))
    }
}

impl Builder<SpecialisationComponent> for ProgramComponentBuilder {
    fn build(json: &serde_json::Map<String, Value>) -> Option<SpecialisationComponent> {
        let major = json.get("majors");
        let minor = json.get("minors");
        let honours = json.get("honours");

        fn build_view(view_json: Option<&Value>) -> Option<HashMap<String, SpecialisationView>> {
            let mut buf = HashMap::new();
            view_json?.as_object()?.iter().for_each(|(key, value)| {
                let direction = key.clone();
                let object = value.as_object().unwrap();
                let mut specialisations = Vec::new();
                object
                    .get("specs")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .keys()
                    .for_each(|key| specialisations.push(key.clone()));
                let notes = object.get("notes").unwrap().as_str().unwrap().to_string();
                let is_optional = object.get("is_optional").unwrap().as_bool().unwrap();
                buf.insert(
                    direction,
                    SpecialisationView::new(specialisations, notes, is_optional),
                );
            });
            Some(buf)
        }
        Some(SpecialisationComponent::new(
            build_view(major),
            build_view(minor),
            build_view(honours),
        ))
    }
}

impl Builder<Rules> for ProgramComponentBuilder {
    fn build(json: &serde_json::Map<String, Value>) -> Option<Rules> {
        let rules_type = json.get("type")?.as_str()?;
        match rules_type {
            "info_rule" => {
                let title = json.get("title")?.as_str()?;
                let notes = json.get("notes")?.as_str()?;
                Some(Rules::Info(InfoRule::new(
                    title.to_string(),
                    notes.to_string(),
                )))
            }
            "limit_rule" => {
                let title = json.get("title")?.as_str()?;
                let notes = json.get("notes")?.as_str()?;
                let course_msgs = json
                    .get("courses")?
                    .as_object()?
                    .values()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join("\n- ");
                let mut buf = notes.to_string();
                buf.push('\n');
                buf.push_str(&course_msgs);
                Some(Rules::Limit(InfoRule::new(title.to_string(), buf)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_json() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        assert_ne!(api.programs.len(), 0);
        assert_ne!(api.specialiastions.len(), 0);
    }

    #[test]
    fn test_get_program() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        assert_eq!(program.title, "Commerce / Computer Science");
        assert_eq!(program.code(), "3784");
        assert_eq!(program.uoc, 192);
        assert_eq!(program.duration, 4);
        let course_components = program.course_components.as_ref().unwrap().clone();
        assert_eq!(course_components.len(), 4);
        let specialisation = program.specialisation_component.as_ref().unwrap().clone();
        assert!(specialisation.honours.is_none());
        assert_eq!(specialisation.major.unwrap().len(), 2);
        assert_eq!(specialisation.minor.unwrap().len(), 1);
    }

    #[test]
    fn test_get_specialisation() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let specialisation = api.get_specialiastion("COMPA1").unwrap();
        assert_eq!(specialisation.name, "Computer Science");
        assert_eq!(specialisation.code(), "COMPA1");
        assert_eq!(specialisation.uoc, 96);
        assert!(specialisation.constraints.is_none());
        assert_eq!(specialisation.course_component().len(), 2)
    }

    #[test]
    fn test_program_coursecomponent() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let course_components = program.course_components.as_ref().unwrap();
        assert_eq!(course_components.len(), 4);
        assert_eq!(
            course_components
                .get("Final Year Synthesis")
                .unwrap()
                .courses
                .len(),
            12
        );
        assert!(course_components
            .get("Final Year Synthesis")
            .unwrap()
            .courses
            .contains(&Course::Course(CourseCode::from_str("ACCT3583").unwrap())));
    }

    #[test]
    fn test_program_rules() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let rules = program.rules.clone();
        assert_eq!(rules.len(), 4);
    }

    #[test]
    fn test_major() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let specialisation = program.specialisation_component.as_ref().unwrap();
        let specialisation_view = specialisation
            .major
            .as_ref()
            .unwrap()
            .get("Computer Science")
            .unwrap();
        assert_eq!(specialisation_view.specialisations.len(), 8);
    }

    #[test]
    fn test_minor() {
        let api = ProgramManager::new(
            "/root/UNSW-HandBookX/backend/data/programsProcessed.json",
            "/root/UNSW-HandBookX/backend/data/specialisationsProcessed.json",
        );
        let program = api
            .get_program(&ProgramCode::from_str("3784").unwrap())
            .unwrap();
        let specialisation = program.specialisation_component.as_ref().unwrap();
        let specialisation_view = specialisation
            .minor
            .as_ref()
            .unwrap()
            .get("Commerce")
            .unwrap();
        assert_eq!(specialisation_view.specialisations.len(), 14);
    }
}
