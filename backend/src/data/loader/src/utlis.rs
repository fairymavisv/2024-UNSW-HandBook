use std::fmt::{Display, Formatter, Debug};


// #[derive(Debug)]
#[derive(Clone)]
pub struct ProgramCode {
    code: [char; 4],
}
impl Display for ProgramCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.iter().collect::<String>())
    }
}

impl Debug for ProgramCode {
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

#[derive(Clone)]
pub struct CourseCode {
    school_code: [char; 4],
    course_code: [char; 4],
    any_school : bool,
    any_level: bool
}

impl Display for CourseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.school_code.iter().collect::<String>(), self.course_code.iter().collect::<String>())
    }
    
}

impl Debug for CourseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.school_code.iter().collect::<String>(), self.course_code.iter().collect::<String>())
        
    }
}




impl CourseCode {
    pub fn new(school_code: [char; 4], course_code: [char; 4]) -> CourseCode {
        CourseCode {
            school_code,
            course_code,
            any_school: false,
            any_level: false
        }
    }
    fn from_str_unsafe(s: &str) -> Self {
        CourseCode::from_str(s).unwrap()
    }

    pub fn new_school_with_level(school_code: [char; 4], level: u8) -> CourseCode {
        let mut course_code: [char; 4] = ['_'; 4];
        course_code[0] = char::from_u32(level as u32).unwrap();
        CourseCode {
            school_code,
            course_code,
            any_school: false,
            any_level: false
        }

    }

    pub fn new_any_school(school_code: [char; 4]) -> CourseCode {
        let mut course_code: [char; 4] = ['0'; 4];
        CourseCode {
            school_code,
            course_code,
            any_school: false,
            any_level: true
        }
    }

    pub fn new_any_school_with_level(level: u8) -> CourseCode {
        let school_code: [char; 4] = ['.'; 4];
        let mut course_code: [char; 4] = ['0'; 4];
        course_code[0] = char::from_u32(level as u32).unwrap();
        CourseCode {
            school_code,
            course_code,
            any_school: true,
            any_level: false
        }
    }

    pub fn parse(s: &str) -> Option<CourseCode> {
        if s.len() == 8 {
            return CourseCode::from_str(s)
        } 
        let mut school_code: [char; 4] = ['.'; 4];
        let mut course_code: [char; 4] = ['0'; 4];
        let mut is_any_school = false;
        let mut num_code = 0;
        for (i, c) in s.chars().enumerate() {
            if i < 4 {
                if c == '.' {
                    is_any_school = true;

                }
                if c.is_ascii_alphabetic() {
                    school_code[i] = c;
                } else {
                    return None
                }
            } else {
                if !c.is_numeric() {
                    return None;
                }
                course_code[i - 4] = c;
                num_code += 1;
            }
        }
        let given_code = num_code == 1;
        Some(CourseCode {
            school_code,
            course_code,
            any_school: is_any_school,
            any_level: !given_code
        })
        // 
        // todo!()

    }

    pub fn from_str(s: &str) -> Option<CourseCode> {
        if s.len() != 8 {
            return None;
        }
        let mut school_code: [char; 4] = ['.'; 4];
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
    pub fn level(&self) -> u8 {
        self.course_code[0].to_digit(10).unwrap() as u8
    }    

    pub fn is_specific_course(&self) -> bool {
        !(self.any_level || self.any_school)
    }
}

impl From<&str> for CourseCode {
    fn from(value: &str) -> Self {
        Self::from_str_unsafe(value)
    }
}

impl PartialEq for CourseCode {
    fn eq(&self, other: &Self) -> bool {

        match ((self.any_school, self.any_level), (other.any_school, other.any_level)) {
            ((true, true), _) => true,
            ((true, false), (true, false)) | ((true, false), (false, false))=> self.level() == self.level(),
            ((true, false), (false, true))  => false,
            ((true, false), (true, true)) => true,
            ((false, _), (true,_ )) => other.eq(self),
            ((false, false), (false, false)) => self.to_string().eq(&other.to_string()),
            _ => self.to_string().eq(&other.to_string())
        }

    }
}

#[derive(Debug, Clone)]
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
impl PartialEq for OfferingTerm {
    fn eq(&self, other: &Self) -> bool {
        match self {
            OfferingTerm::Summer => match other {
                OfferingTerm::Summer => true,
                _ => false
            },
            OfferingTerm::Term1 => match other {
                OfferingTerm::Term1 => true,
                _ => false
            },
            OfferingTerm::Term2 => match other {
                OfferingTerm::Term2 => true,
                _ => false
            },
            OfferingTerm::Term3 => match other {
                OfferingTerm::Term3 => true,
                _ => false
            },

        }
    }
}





#[derive(Clone)]
pub enum Campus {
    Sydney,
    Paddington,
    Canberra,
}
impl Campus {
    pub fn from_str(s: &str) -> Option<Campus> {
        match s {
            "Sydney" => Some(Campus::Sydney),
            "UNSW Canberra" => Some(Campus::Canberra),
            "Paddington" => Some(Campus::Paddington),
            _ => None,
        }
    }
    
}

#[derive(Clone)]
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