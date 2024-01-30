use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

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

impl PartialEq for ProgramCode {
    fn eq(&self, other: &Self) -> bool {
        self.code.iter().zip(other.code.iter()).all(|(a, b)| a == b)
    }
}

impl ProgramCode {
    pub fn new(code: [char; 4]) -> ProgramCode {
        ProgramCode { code }
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
    num_of_match_school: u8,
    num_of_match_code: u8,
}

impl Display for CourseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.school_code.iter().collect::<String>(),
            self.course_code.iter().collect::<String>()
        )
    }
}

impl Debug for CourseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.school_code.iter().collect::<String>(),
            self.course_code.iter().collect::<String>()
        )
    }
}
impl Eq for CourseCode {
    fn assert_receiver_is_total_eq(&self) {
        //
    }
}

impl Hash for CourseCode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl CourseCode {
    pub fn new(school_code: [char; 4], course_code: [char; 4]) -> CourseCode {
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 4,
            num_of_match_code: 4,
        }
    }
    fn from_str_unsafe(s: &str) -> Self {
        CourseCode::from_str(s).unwrap()
    }

    pub fn new_school_with_level(school_code: [char; 4], level: u8) -> CourseCode {
        let mut course_code: [char; 4] = ['#'; 4];
        course_code[0] = char::from_u32(level as u32).unwrap();
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 4,
            num_of_match_code: 1,
        }
    }

    pub fn new_any_school(school_code: [char; 4]) -> CourseCode {
        let mut course_code: [char; 4] = ['#'; 4];
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 4,
            num_of_match_code: 0,
        }
    }

    pub fn new_any_school_with_level(level: u8) -> CourseCode {
        let school_code: [char; 4] = ['.'; 4];
        let mut course_code: [char; 4] = ['#'; 4];
        course_code[0] = char::from_u32(level as u32).unwrap();
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 0,
            num_of_match_code: 1,
        }
    }

    pub fn is_pattern(&self) -> bool {
        self.num_of_match_school != 4 || self.num_of_match_code != 4
    }

    pub fn is_course_code(&self) -> bool {
        self.num_of_match_school == 4 && self.num_of_match_code == 4
    }

    pub fn parse(s: &str) -> Option<CourseCode> {
        if s.len() == 8 {
            return CourseCode::from_str(s);
        }
        let mut school_code: [char; 4] = ['.'; 4];
        let mut course_code: [char; 4] = ['#'; 4];
        let mut num_of_match_school = 0;
        let mut num_of_match_code = 0;
        for (i, c) in s.chars().enumerate() {
            if i < 4 {
                if c.is_ascii_alphabetic() {
                    school_code[i] = c;
                    num_of_match_school += 1;
                }
            } else {
                if !c.is_numeric() {
                    return None;
                }
                course_code[i - 4] = c;
                num_of_match_code += 1;
            }
        }
        Some(CourseCode {
            school_code,
            course_code,
            num_of_match_school,
            num_of_match_code,
        })
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
}

impl From<&str> for CourseCode {
    fn from(value: &str) -> Self {
        Self::from_str_unsafe(value)
    }
}

impl PartialEq for CourseCode {
    fn eq(&self, other: &Self) -> bool {
        self.school_code
            .iter()
            .zip(other.school_code.iter())
            .take(self.num_of_match_school as usize)
            .all(|(a, b)| a == b)
            && self
                .course_code
                .iter()
                .zip(other.course_code.iter())
                .take(self.num_of_match_code as usize)
                .all(|(a, b)| a == b)
    }
}

#[derive(Debug, Clone)]
pub enum OfferingTerm {
    Term1,
    Term2,
    Term3,
    Summer,
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

impl Display for OfferingTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OfferingTerm::Summer => write!(f, "Summer Term"),
            OfferingTerm::Term1 => write!(f, "Term 1"),
            OfferingTerm::Term2 => write!(f, "Term 2"),
            OfferingTerm::Term3 => write!(f, "Term 3"),
        }
    }
}
impl PartialEq for OfferingTerm {
    fn eq(&self, other: &Self) -> bool {
        match self {
            OfferingTerm::Summer => match other {
                OfferingTerm::Summer => true,
                _ => false,
            },
            OfferingTerm::Term1 => match other {
                OfferingTerm::Term1 => true,
                _ => false,
            },
            OfferingTerm::Term2 => match other {
                OfferingTerm::Term2 => true,
                _ => false,
            },
            OfferingTerm::Term3 => match other {
                OfferingTerm::Term3 => true,
                _ => false,
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
pub enum StudyLevel {
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
