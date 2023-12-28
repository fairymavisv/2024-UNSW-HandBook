use std::fmt::{Display, Formatter};


pub struct ProgramCode {
    code: [char; 4],
}
impl Display for ProgramCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.iter().collect::<String>())
    }
}

impl ProgramCode {
    pub fn new(code: [char; 4]) -> ProgramCode {
        ProgramCode {
            code,
        }
    }

    pub fn from_str(s: &str) -> Option<ProgramCode> {
        if s.len() != 4 {
            return None;
        }
        let mut code: [char; 4] = ['0'; 4];
        for (i, c) in s.chars().enumerate() {
            if !c.is_numeric() {
                return None;
            }
            code[i] = c;
        }
        Some(ProgramCode::new(code))
    }

    pub fn is_code(s: &str) -> bool {
        ProgramCode::from_str(s).is_some()
    }
}

pub struct CourseCode {
    school_code: [char; 4],
    course_code: [char; 4],
}

impl Display for CourseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.school_code.iter().collect::<String>(), self.course_code.iter().collect::<String>())
    }
    
}

impl CourseCode {
    pub fn new(school_code: [char; 4], course_code: [char; 4]) -> CourseCode {
        CourseCode {
            school_code,
            course_code,
        }
    }
    pub fn from_str(s: &str) -> Option<CourseCode> {
        if s.len() != 8 {
            return None;
        }
        let mut school_code: [char; 4] = ['0'; 4];
        let mut course_code: [char; 4] = ['0'; 4];
        for (i, c) in s.chars().enumerate() {
            if i < 4 {
                school_code[i] = c;
            } else {
                if !c.is_numeric() {
                    return None;
                }
                course_code[i - 4] = c;
            }
        }
        Some(CourseCode::new(school_code, course_code))
    }

    pub fn is_code(s: &str) -> bool {
        CourseCode::from_str(s).is_some()
    }
    pub fn school_code(&self) -> &[char; 4] {
        &self.school_code
    }
    pub fn course_code(&self) -> &[char; 4] {
        &self.course_code
    }
    
}

pub enum OfferingTerm{
    Term1,
    Term2,
    Term3,
    Summer
}

impl OfferingTerm {
    pub fn from_str(s: &str) -> Option<OfferingTerm> {
        match s {
            "T1" => Some(OfferingTerm::Term1),
            "T2" => Some(OfferingTerm::Term2),
            "T3" => Some(OfferingTerm::Term3),
            "T0" => Some(OfferingTerm::Summer),
            _ => None,
        }
    }
}



pub enum Campus {
    Sydney,
    Canberra,
}
impl Campus {
    pub fn from_str(s: &str) -> Option<Campus> {
        match s {
            "Sydney" => Some(Campus::Sydney),
            "Canberra" => Some(Campus::Canberra),
            _ => None,
        }
    }
    
}

pub enum  StudyLevel {
    Undergraduate,
    Postgraduate,
    
}
impl StudyLevel {
    pub fn from_str(s: &str) -> Option<StudyLevel> {
        match s {
            "Undergraduate" => Some(StudyLevel::Undergraduate),
            "Postgraduate" => Some(StudyLevel::Postgraduate),
            _ => None,
        }
    }
    
}