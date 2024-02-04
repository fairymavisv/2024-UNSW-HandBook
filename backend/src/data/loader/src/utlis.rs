/// This module contains the definition of the `CourseCode`, `ProgramCode`, `OfferingTerm`, `Campus`, and `StudyLevel`
use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

// #[derive(Debug)]
/// This struct represents a program code
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
    /// Create a new `ProgramCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `code` - A 4-character array representing the program code
    /// 
    /// # Returns
    /// 
    /// A `ProgramCode` instance
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = ProgramCode::new(['Z', 'C', 'S', '1']);
    /// ```
    pub fn new(code: [char; 4]) -> ProgramCode {
        ProgramCode { code }
    }

    /// Parse a string into a `ProgramCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `s` - A string slice representing the program code
    /// 
    /// # Returns
    /// 
    /// A `ProgramCode` instance
    /// None if the string is not a valid program code
    /// 
    /// # Example
    /// 
    /// ```
    /// 
    /// let code = ProgramCode::from_str("ZCS1").unwrap();
    /// ```
    /// 
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

    /// Check if a string is a valid program code
    pub fn is_code(s: &str) -> bool {
        ProgramCode::from_str(s).is_some()
    }
}


/// This struct represents a course code
/// 
/// It contains the school code and the course code
/// 
/// It could represent a pattern or a specific course code
/// 
/// Specific course code has 4 alphabetic characters for the school code and 4 numeric characters for the course code
/// 
/// Pattern course code has 0-4 alphabetic characters for the school code and 0-4 numeric characters for the course code.
/// Fill the rest with '.' and '#' for the school code and course code respectively if the length is less than 4.
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
    /// Create a new `CourseCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `school_code` - A 4-character array representing the school code
    /// * `course_code` - A 4-character array representing the course code
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    pub fn new(school_code: [char; 4], course_code: [char; 4]) -> CourseCode {
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 4,
            num_of_match_code: 4,
        }
    }

    /// Parse a string into a `CourseCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `s` - A string slice representing the course code
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = CourseCode::from_str("ZCSC1234").unwrap();
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if the string is not a valid course code, i.e. it does not a specific course code.
    pub fn from_str_unsafe(s: &str) -> Self {
        CourseCode::from_str(s).unwrap()
    }

    /// Create a new `CourseCode` instance with a specific school code and a level
    /// 
    /// # Arguments
    /// 
    /// * `school_code` - A 4-character array representing the school code
    /// * `level` - A number representing the level
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    /// 
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

    /// Create a new `CourseCode` instance with a pattern school code and a specific course code
    /// 
    /// # Arguments
    /// 
    /// * `course_code` - A 4-character array representing the course code
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    pub fn new_any_school(school_code: [char; 4]) -> CourseCode {
        let mut course_code: [char; 4] = ['#'; 4];
        CourseCode {
            school_code,
            course_code,
            num_of_match_school: 4,
            num_of_match_code: 0,
        }
    }

    /// Create a new `CourseCode` instance with a pattern school code and a pattern course code
    /// 
    /// # Arguments
    /// 
    /// * `level` - A number representing the level
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
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

    /// Check if the `CourseCode` instance is a pattern
    pub fn is_pattern(&self) -> bool {
        self.num_of_match_school != 4 || self.num_of_match_code != 4
    }

    /// Check if the `CourseCode` instance is a specific course code
    pub fn is_course_code(&self) -> bool {
        self.num_of_match_school == 4 && self.num_of_match_code == 4
    }

    /// Parse a string into a `CourseCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `s` - A string slice representing the course code
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    /// None if the string is not a valid course code, i.e. it is not either a specific course code or a valid pattern.
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = CourseCode::parse("ZCSC1234").unwrap();
    /// let code = CourseCode::parse("ZC..1").unwrap(); 
    /// let code = CourseCode::parse("ZCSC12").unwrap(); 
    /// let code = CourseCode::parse("....").unwrap(); // all course in UNSW
    /// let code = CourseCode::parse("").unwrap(); // all course in UNSW
    /// ```
    /// # Note
    /// `s` must at least have 4 characters if required any numeric pattern
    /// 
    /// This function is more flexible than `from_str` as it can parse a pattern course code, 
    /// please use this function if you are not sure if the string is a specific course code or a pattern.
    pub fn parse(s: &str) -> Option<CourseCode> {
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

    /// Parse a string into a `CourseCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `s` - A string slice representing the course code
    /// 
    /// # Returns
    /// 
    /// A `CourseCode` instance
    /// None if the string is not a valid course code, i.e it is not a specific course code, 4 alphabetic characters and 4 numeric characters.
    /// 
    /// # Example
    /// 
    /// ```
    /// let code = CourseCode::from_str("ZCSC1234").unwrap();
    /// ```
    /// 
    /// # Note
    /// This function is more strict than `parse` as it can only parse a specific course code.
    pub fn from_str(s: &str) -> Option<CourseCode> {
        if s.len() != 8 {
            return None;
        }
        let mut school_code: [char; 4] = ['.'; 4];
        let mut course_code: [char; 4] = ['#'; 4];
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

    /// Check if a string is a valid course code
    pub fn is_code(s: &str) -> bool {
        CourseCode::from_str(s).is_some()
    }

    /// Get the school code, i.e. the first 4 characters of the course code
    pub fn school_code(&self) -> &[char; 4] {
        &self.school_code
    }

    /// Get the course code, i.e. the last 4 characters of the course code/ numeric characters
    pub fn course_code(&self) -> &[char; 4] {
        &self.course_code
    }

    /// Get the level of the course code
    pub fn level(&self) -> u8 {
        self.course_code[0].to_digit(10).unwrap() as u8
    }

    /// Adjust the pattern of the course code
    /// 
    /// # Arguments
    /// 
    /// * `num_of_match_school_code` - A number representing the number of characters to match for the school code
    /// * `num_of_match_course_code` - A number representing the number of characters to match for the course code
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut code = CourseCode::from_str("ZCSC1234").unwrap();
    /// 
    /// code.adjust_pattern(2, 3);
    /// ```
    /// 
    /// # Note
    /// This function is useful when you want to adjust the pattern of the course code.
    pub fn adjust_pattern(&mut self, num_of_match_school_code: u8, num_of_match_course_code: u8) {
        if num_of_match_school_code > 4 || num_of_match_course_code > 4 {
            return;
        }
        if self.num_of_match_code == num_of_match_course_code
            && self.num_of_match_school == num_of_match_school_code
        {
            return;
        }

        if self.num_of_match_code != num_of_match_course_code {
            let mut course_code: [char; 4] = ['#'; 4];
            for (i, c) in self.course_code.iter().enumerate() {
                if i < num_of_match_course_code as usize {
                    course_code[i] = *c;
                }
            }
            self.course_code = course_code;
            self.num_of_match_code = num_of_match_course_code;
        }
        if self.num_of_match_school != num_of_match_school_code {
            let mut school_code: [char; 4] = ['.'; 4];
            for (i, c) in self.school_code.iter().enumerate() {
                if i < num_of_match_school_code as usize {
                    school_code[i] = *c;
                }
            }
            self.school_code = school_code;
            self.num_of_match_school = num_of_match_school_code;
        }
    }

    /// Check if the `CourseCode` instance is a match of another `CourseCode` instance
    /// 
    /// # Arguments
    /// 
    /// * `other` - Another `CourseCode` instance
    /// 
    /// # Returns
    /// 
    /// A boolean value
    pub fn is_match(&self, other: &CourseCode) -> bool {
        self.eq(other)
    }
}

impl From<&str> for CourseCode {
    fn from(value: &str) -> Self {
        Self::from_str_unsafe(value)
    }
}

impl PartialEq for CourseCode {
    fn eq(&self, other: &Self) -> bool {
        if self.num_of_match_school == other.num_of_match_school
            && self.num_of_match_code == other.num_of_match_code
        {
            return self.to_string() == other.to_string();
        }
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

/// This enum represents the offering term
#[derive(Debug, Clone)]
pub enum OfferingTerm {
    Term1,
    Term2,
    Term3,
    Summer,
}


impl OfferingTerm {
    /// Parse a string into a `OfferingTerm` instance
    /// 
    /// T1 -> Term1
    /// 
    /// T2 -> Term2
    /// 
    /// T3 -> Term3
    /// 
    /// T0 -> Summer
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

/// This enum represents the campus
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

/// This enum represents the study level
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
