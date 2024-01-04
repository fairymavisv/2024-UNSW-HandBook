use core::num;
use std::fmt::Display;

use rayon::iter;
use wasm_bindgen::convert::OptionIntoWasmAbi;

use crate::{course::{Course, CourseManager, self}, utlis::{CourseCode, ProgramCode}, program};

pub struct Requirements {
    contents: Option<Box<dyn Node + Send>>

}

impl Display for Requirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.contents.is_none() {
            write!(f, "")
        } else {
            write!(f, "{}", self.contents.as_ref().unwrap().get())
        }
    }
    
}

#[derive(Debug)]
pub enum Keyword {
    LEVEL,
    UOC,
    WAM,
}
#[derive(Debug)]
pub enum Operator {
    OR,
    AND,
}

#[derive(Debug)]
pub enum Preposition {
    AT,
    FROM,
    OF
}
#[derive(Debug)]
pub enum Code {
    COURSE(CourseCode),
    PROGRAM(ProgramCode),
}
#[derive(Debug)]
pub enum Bracket {
    OPEN,
    CLOSE,
}
#[derive(Debug)]
pub enum Token {
    COMMA,
    BRACKET(Bracket),
    NUMBER(u8),
    KEYWORD(Keyword),
    OPERATOR(Operator),
    PREPOSITION(Preposition),
    CODE(Code),
    TEXT(String),
}

macro_rules! extract_prerequisite {
    ($line:ident, $prerequisite:expr) => {
        {
            if $line.starts_with($prerequisite) {
                return $line.replace($prerequisite, "").trim().to_string();
            }
        }
    };
}


fn print_buffer(buf: &Vec<Box<dyn Node + Send>>) {
    println!("Buffer tokens: {}", buf.iter().map(|node| node.get()).collect::<Vec<String>>().join(", "))

}


impl Requirements {
    pub fn try_new(raw_requirements: &str) -> Option<Requirements> {
        let cleaned_requirements = Requirements::clean(raw_requirements);
        let mut tokens = Requirements::tokenize(cleaned_requirements.as_str());
        let node = Requirements::parse(&mut tokens);
        if node.is_err() {
            println!("{}", node.err().unwrap());
            None
        } else {
            Some(Requirements {
                contents: node.unwrap()
            })
        }
    }

    fn clean(raw_requirements: &str) -> String {
        let cleaned_lines = raw_requirements.trim().replace("[", " [ ")
            .replace("]", " ] ")
            .replace("(", " ( ")
            .replace(")", " ) ")
            .replace(",", " , ")
            .replace(".", " ")
            .replace("+", "")
            .replace("<br/>", "\n");
        for line in cleaned_lines.lines() {
            let trimed = line.trim();
            extract_prerequisite!(trimed, "Pre-requisites:");
            extract_prerequisite!(trimed, "Pre-requisite:");
            extract_prerequisite!(trimed, "Prerequisites:");
            extract_prerequisite!(trimed, "Prerequisite:");
        }
        String::from("")
            
    }

    fn tokenize(raw_requirements: &str) -> Vec<Token> {
        // let cleanned_requirements = Requirements::clean(raw_requirements);
        let mut tokens: Vec<Token> = Vec::new();
        let mut itr = raw_requirements.split_ascii_whitespace();
        loop {
            let word = itr.next();
            if word.is_none() {
                break;
            }
            match word.unwrap() {
                "or" | "OR" | "Or" => {
                    tokens.push(Token::OPERATOR(Operator::OR));
                },
                "and" | "AND" | "And" => {
                    tokens.push(Token::OPERATOR(Operator::AND));
                },
                "UOC" | "uoc" | "Uoc" | "UOCs" => {
                    tokens.push(Token::KEYWORD(Keyword::UOC));
                },
                "WAM" | "wam" | "Wam" => {
                    tokens.push(Token::KEYWORD(Keyword::WAM));
                },
                "at" | "AT" => {
                    tokens.push(Token::PREPOSITION(Preposition::AT));
                },
                "from" | "FROM" => {
                    tokens.push(Token::PREPOSITION(Preposition::FROM));
                },
                "of" | "OF" => {
                    tokens.push(Token::PREPOSITION(Preposition::OF));
                },
                "level" | "LEVEL" => {
                    tokens.push(Token::KEYWORD(Keyword::LEVEL));
                },
                "," => {
                    tokens.push(Token::COMMA);
                },
                "(" => {
                    tokens.push(Token::BRACKET(Bracket::OPEN));
                },
                ")" => {
                    tokens.push(Token::BRACKET(Bracket::CLOSE));
                },
                str => {
                    if str.eq("TEXT") {
                        let mut str = Vec::new();
                        let mut start = false;
                        loop {
                            let next = itr.next();
                            if next.is_none() {
                                break;
                            }
                            let next_str = next.unwrap();
                            if next_str.eq("[") {
                                start = true;
                                continue;
                            }
                            if start && next_str.eq("]") {
                                break;
                            } else if start {
                                str.push(next_str.to_string());
                            }
                        }
                        tokens.push(Token::TEXT(str.join(" ")));
                    }
                    if CourseCode::is_code(str) {
                        tokens.push(Token::CODE(Code::COURSE(CourseCode::from_str(str).unwrap())));
                    } else if ProgramCode::is_code(str) {
                        tokens.push(Token::CODE(Code::PROGRAM(ProgramCode::from_str(str).unwrap())));
                    } else if str.parse::<u8>().is_ok() {
                        tokens.push(Token::NUMBER(str.parse::<u8>().unwrap()));
                    } 
                }

            }
        
        }
        println!("Tokens: {:?}", tokens);
        tokens
    }



    fn parse(tokens: &mut Vec<Token>) -> Result<Option<Box<dyn Node + Send>>, String> {
        // if tokens.len() == 0 {
        //     return Ok(None);
        // }
        tokens.reverse();
        Requirements::do_parse(tokens)
    }

    fn do_parse(tokens: &mut Vec<Token>) -> Result<Option<Box<dyn Node + Send>>, String> {
        // let mut tokens = Requirements::tokenize(raw_requirements);
        // let requirement = Requirements::new(String::from("raw_requirements"));
        if tokens.len() == 0 {
            return Ok(None);
        }
        let mut buffer: Vec<Box<dyn Node + Send>> = Vec::new();
        let mut parsed_node: Vec<Box<dyn Node + Send>> = Vec::new();
        // tokens.reverse();
        
        loop {
            let token = tokens.pop();
            if token.is_none() {
                break;
            }
            match token.unwrap() {
                Token::TEXT(text) => {
                    println!("{}", text);
                    buffer.push(Box::new(TextNode::new(text)));
                },
                Token::CODE(code) => {
                    println!("{:?}", code);
                    buffer.push(Box::new(CodeNode::new(code)));
                },
                Token::OPERATOR(operator) => {
                    println!("line 239 tokens {:?}", operator);
                    if buffer.len() == 1 {
                        let left = buffer.pop().unwrap();
                        // print_buffer(tokens);
                        let mut sub_tokens: Vec<Token>= Vec::new();
                        loop {
                            let token = tokens.pop();
                            if token.as_ref().is_none() {
                                break;
                            } else {
                                if let Token::COMMA = token.as_ref().unwrap() {
                                    tokens.push(Token::COMMA);
                                    break;
                                } 
                            }
                            sub_tokens.push(token.unwrap());

                        }
                        let right = Requirements::parse(&mut sub_tokens)?;
                        if right.is_none() {
                            buffer.push(left);
                        } else {
                            let binary = BinaryNode::new(left, right.unwrap(), operator);
                            buffer.push(Box::new(binary));
                        }
                    } else if buffer.len() == 0 {
                        println!("Warning (BIN-1): Binary operator (e.g and, or) without left hand side");
                        continue;
                    } else {
                        print_buffer(&buffer);
                        return Err(String::from("ERROR (BIN-1): more than one left hand side token."));
                    }
                },
                Token::NUMBER(number) => {
                    println!("{}", number);
                    let next = tokens.pop();
                    if next.is_none() {
                        println!("Warning (NUM-1): Number without following token.");
                        break;
                    }
                    match next.unwrap() {
                        Token::KEYWORD(keyword) => {
                            match keyword {
                                Keyword::UOC => { 
                                    let position = tokens.last();
                                    if position.is_none() {
                                        buffer.push(Box::new(UOCNode::new(number)));
                                        continue;
                                    }
                                    match position.unwrap() {
                                        Token::PREPOSITION(preposition) => {
                                            // tokens.pop();
                                            match preposition {
                                                Preposition::FROM => {
                                                    let mut node = Box::new(UOCFromNode::new(number ));
                                                    println!("line 275 Tokens: {:?}", tokens);
                                                    if node.parse(tokens)? == () {
                                                        buffer.push(node);

                                                    }
                                                },
                                                Preposition::AT => {
                                                    let mut node = Box::new(UOCAtLevelNode::new(number ));
                                                    if node.parse(tokens)? == () {
                                                        buffer.push(node);
                                                    }
                                                },
                                                _ => println!("Warning (UOC-1): Only FROM and AT can be followed by a UOC number")
                                            }
                                        },
                                        _ => println!("Warning (UOC-1): Only preposition (FROM and AT) can be followed by a UOC number")
                                    }

                                },
                                Keyword::WAM => {
                                    buffer.push(Box::new(WamNode::new(number)));
                                },
                                _ => println!("Warning (NUM-2): Only WAM and UOC can be followed by a number"),
                                
                            }
                        },
                        _ => {
                            println!("Warning (NUM-3): Only Keywords WAM and UOC can be followed by a number");
                        }
                    }

                },
                Token::COMMA => {
                    if buffer.len() == 1 {
                        parsed_node.push(buffer.pop().unwrap());

                    } else if buffer.len() > 1 {
                        // dbg!(buffer);
                        print_buffer(&buffer);
                        return Err(String::from("ERROR (COM-1): There are more than one node in the buffer."));
                    }                     
                    println!(",");
                },
                Token::BRACKET(bracket) => {
                    println!("{:?}", bracket);
                    let mut num_bracket = 1;
                    let mut sub_token: Vec<Token> = Vec::new();
                    // sub_token.push(Token::BRACKET(bracket));
                    while num_bracket != 0 {
                        let token = tokens.pop();
                        if let Some(token) = token {
                            match &token {
                                Token::BRACKET(bracket) => {
                                    match bracket {
                                        Bracket::OPEN => num_bracket += 1,
                                        Bracket::CLOSE => num_bracket -= 1
                                    }
                                }
                                _ => ()
                            }
                            sub_token.push(token);
                        } else {
                            return Err(String::from("ERROR (BRA-1): Unclose bracket."));
                        }
                    }
                    sub_token.pop();
                    println!("Sub Token{:?}", sub_token);
                    let node = Requirements::parse(&mut sub_token)?;
                    if let Some(node) = node {
                        buffer.push(node);
                    }

                },
                Token::KEYWORD(keyword) => {
                    match keyword {
                        Keyword::WAM => {
                            let mut node = Box::new(WamNode::new(0));
                            if node.parse(tokens)? == () {
                                buffer.push(node);
                            }
                        },
                        _ => println!("Warning (KEY-1): Keyword {} is not supported yet", "keyword"),
                    }
                    println!("{:?}", keyword);
                },
                _  => ()
            }
        }
        if buffer.len() == 1 {
            parsed_node.push(buffer.pop().unwrap());
        } else if buffer.len() > 1 {
            print_buffer(&buffer);
            return Err(String::from("ERROR (COM-1): There are more than one node in the buffer."));
        }

        if parsed_node.len() == 1 {
            return Ok(Some(parsed_node.pop().unwrap()));
        } else if parsed_node.len() > 1 {
            return Ok(Some(Box::new(ListNode::new_from_nodes(parsed_node))));
        } else {
            return Ok(None);
        }
    }
}

pub trait Node {
    fn parse(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
        Ok(())
    }

    fn get(&self) -> String;
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String>;

}



// #[derive(Debug)]

// node, node, node
struct ListNode {
    nodes: Vec<Box<dyn Node + Send>>,
}

impl ListNode {
    fn new(nodes: &mut Vec<Box<dyn Node + Send>>) -> Self {
        ListNode {
            nodes: nodes.drain(..).collect(),
        }
    }
    fn new_from_nodes(nodes: Vec<Box<dyn Node + Send>>) -> Self {
        ListNode {
            nodes,
        }
    }
}

impl Node for ListNode {
    fn get(&self) -> String {
        format!("Require all of following conditions: {}", self.nodes.iter().map(|node| node.get()).collect::<Vec<String>>().join(", "))
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        Ok(self.nodes.iter().all(|node| node.evulate(program_code, taken_course, wam, course_manager).unwrap_or(false)))
            
    }
}

// TODO 
// BABS2204 OR BABS2264 OR BIOC2201
// Node or Node and Node, And Node
// (BABS2204 OR (BABS2264 OR BIOC2201)) and (BABS2204 OR (BABS2264 OR BIOC2201)
// OR OR BABS2204 BABS2264  BIOC2201
pub struct BinaryNode {
    left: Box<dyn Node + Send>,
    right: Box<dyn Node + Send>,
    operator: Operator,
}

impl BinaryNode {
    pub fn new(left: Box<dyn Node + Send>, right: Box<dyn Node + Send>, operator: Operator) -> BinaryNode {
        BinaryNode {
            left,
            right,
            operator,
        }
    }
    // fn parse(&mut self, tokens: &mut Vec<Box<dyn Node>>) -> bool {
    //     true
    // }
}

impl Node for BinaryNode {
    fn get(&self) -> String {
        format!("({} {} {})", self.left.get(), match self.operator {
            Operator::AND => "and",
            Operator::OR => "or",
        }, self.right.get())
    
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        match self.operator {
            Operator::AND => {
                let left = self.left.evulate(program_code, taken_course, wam, course_manager)?;
                if left == false {
                    return Ok(false)
                }
                let right = self.right.evulate(program_code, taken_course, wam, course_manager)?;
                if right == false {
                    return Ok(false)
                }
                return Ok(true)
            },
            Operator::OR => {
                let left = self.left.evulate(program_code, taken_course, wam, course_manager)?;
                if left == true {
                    return Ok(true)
                }
                let right = self.right.evulate(program_code, taken_course, wam, course_manager)?;
                if right == true {
                    return Ok(true)
                }
                return Ok(false)

            }
        }
    }
}


//  UOC at level x
struct UOCAtLevelNode {
    uoc: u8,
    level: u8,
}
impl UOCAtLevelNode {
    fn new(uoc: u8) -> UOCAtLevelNode {
        UOCAtLevelNode {
            uoc,
            level: 0,
        }
    }
}
impl Node for UOCAtLevelNode {

    fn parse(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
        tokens.pop();
        let token = tokens.pop();
        if token.is_none() {
            return Err(String::from("ERROR (UOC-2A): UOC at level without following level keyword"));
        }
        match token.unwrap() {
            Token::KEYWORD(keyword) => {
                match keyword {
                    Keyword::LEVEL => {
                        let token = tokens.pop();
                        if token.is_none() {
                            return Err(String::from("ERROR (UOC-2B): UOC at level without following level number"));
                        }

                        match token.unwrap() {
                            Token::NUMBER(level) => {
                                self.level = level;
                                return Ok(());
                            },
                            _ => return Err(String::from("ERROR (UOC-2C): UOC at level without following level number")),
                        }
                    },
                    _ => return Err(String::from("ERROR (UOC-2D): UOC at level without following level keyword")),
                }
            },
            _ => return Err(String::from("ERROR (UOC-2E): UOC at level without following level keyword")),
        }
    }

    fn get(&self) -> String {
        format!("Complete {} UOC at level {}", self.uoc, self.level)
    }

    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        let mut sum = 0;
        for course in taken_course {
            let course_code = CourseCode::from_str(&course);
            if course_code.as_ref().is_none() {
                return Err(String::from(format!("Given {} is not a Course Code", course)));
            }
            if course_code.as_ref().unwrap().level() == self.level {
                sum += course_manager.get_course(&course_code.unwrap())?.uoc()
            }
        }
        if sum >= self.uoc {
            Ok(true)
        } else {
            Ok(false)
        }
        // .map(|course| course.)
    }
}

//  UoC from XXXXX 
pub struct UOCFromNode {
    uoc: u8,
    code: Vec<CourseCode>,
}
impl UOCFromNode {
    fn new(uoc: u8) -> UOCFromNode {
        UOCFromNode {
            uoc,
            code: Vec::new(),
        }
    }
}

impl Node for UOCFromNode {
    fn parse(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
        tokens.pop();
        println!("line 520 Tokens: {:?}", tokens);
        loop {
            let token = tokens.pop();
            if token.is_none() {
                break;
            }
            match token.unwrap() {
                Token::CODE(code) => {
                    match code {
                        Code::COURSE(course_code) => {
                            self.code.push(course_code);
                        },
                        _ => return Err(String::from("ERROR (UOC-4): One of the course code is not a course code")),
                        
                    }
                },
                Token::OPERATOR(operator) => {
                    match operator {
                        Operator::OR => {
                            continue;
                        },
                        _ => {
                            println!("Warning (UOC-1): Course code is not followed by OR operator");
                            break;
                        }
                    }
                },
                _ => return Err(String::from("ERROR (UOC-6): The expected token is a course code or OR operator")),
            }
        }   
        if self.code.len() == 0 {
            return Err(String::from("ERROR (UOC-3A): UOC from without following course code"));
        }
        Ok(())     
    }

    fn get(&self) -> String {
        format!("Complete {} UOC from following course [{}]", self.uoc, self.code.iter().map(|code| code.to_string()).collect::<Vec<String>>().join(", "))
    
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        let mut sum = 0;
        for course in taken_course {
            let course_code = CourseCode::from_str(&course);
            if course_code.as_ref().is_none() {
                return Err(String::from(format!("Given {} is not a Course Code", course)));
            }
            if self.code.contains(&course_code.as_ref().unwrap()) {
                sum += course_manager.get_course(&course_code.unwrap())?.uoc()
            } 

            
        }
        if sum >= self.uoc {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
}

pub struct UOCNode {
    uoc: u8,
}

impl UOCNode {
    pub fn new(uoc: u8) -> UOCNode {
        UOCNode {
            uoc,
        }
    }
}
impl Node for UOCNode {
    fn get(&self) -> String {
        format!("Complete at least {} UOC", self.uoc)
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        let mut sum = 0;
        for course in taken_course {
            let course_code = CourseCode::from_str(&course);
            if course_code.is_none() {
                return Err(String::from(format!("Given {} is not a Course Code", course)));
            }
            sum += course_manager.get_course(&course_code.unwrap())?.uoc()

            
        }
        if sum >= self.uoc {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// WAM of 
pub struct WamNode {
    wam: u8,
}

impl WamNode {
    pub fn new(wam: u8) -> WamNode {
        WamNode {
            wam,
        }
    }
}
impl Node for WamNode {
    fn parse(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
        let last = tokens.last();
        if last.is_none() {
            return Err(String::from("ERROR (WAM-1): WAM without following course code"));
        }
        if let Token::PREPOSITION(Preposition::OF) = last.unwrap() {
            tokens.pop();
        } else {
            return Err(String::from("ERROR (WAM-2): WAM without following OF preposition"));
        }
        let wam = tokens.pop();
        if wam.is_none() {
            return Err(String::from("ERROR (WAM-3): WAM without following WAM number"));
        }
        if let Token::NUMBER(num) = wam.unwrap() {
            self.wam = num;
        }
        Ok(())
    
    }

    fn get(&self) -> String {
        format!("WAM of at least {}", self.wam)
    }

    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        if wam.is_none() {
            Ok(true)
        } else {
            Ok(wam.unwrap() >= self.wam)
        }
    }
}

pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: String) -> TextNode {
        TextNode {
            text,
        }
    }
}

impl Node for TextNode {
    fn get(&self) -> String {
        self.text.clone()
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        Ok(true)
    }
}

pub struct CodeNode {
    code: Code,
}

impl CodeNode {
    pub fn new(code: Code) -> CodeNode {
        CodeNode {
            code,
        }
    }
}

impl Node for CodeNode {
    fn get(&self) -> String {
        match &self.code {
            Code::COURSE(course_code) => {
                format!("{}", course_code.to_string())
            },
            Code::PROGRAM(program_code) => {
                format!("{}", program_code.to_string())
            }
        }
    
    }
    fn evulate(&self, program_code: &str, taken_course: &Vec<String>, wam: &Option<u8>, course_manager: &CourseManager) -> Result<bool, String> {
        match &self.code {
            Code::COURSE(course) => {
                Ok(taken_course.contains(&course.to_string()))
            },
            Code::PROGRAM(program) => {
                Ok(program_code.to_string().eq(&program.to_string()))
            }
        }
    }
}


// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wam_of_node() {
        let requirements = Requirements::try_new("Prerequisites: WAM of 65").unwrap();
        assert_eq!(requirements.to_string(), "WAM of at least 65");

    }

    #[test]
    fn test_new_wam_node() {
        let requirements = Requirements::try_new("Prerequisites: 75 WAM").unwrap();
        assert_eq!(requirements.to_string(), "WAM of at least 75");

    }


    #[test]
    fn test_coursecode_node() {
        let requirements = Requirements::try_new("Prerequisites: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "COMP1511");

    }
    #[test]
    fn test_programcode_node() {
        let requirements = Requirements::try_new("Prerequisites: must enroll in master of commerce - Finance 9999").unwrap();
        assert_eq!(requirements.to_string(), "9999");

    }

    #[test]
    fn test_text_node() {
        let requirements = Requirements::try_new("Prerequisites: TEXT [ major in FINSXXXX]").unwrap();
        assert_eq!(requirements.to_string(), "major in FINSXXXX")
    }

    #[test]
    fn text_uoc_at() {
        let requirements = Requirements::try_new("Prerequisites: 36 UOC at level 1").unwrap();
        assert_eq!(requirements.to_string(), "Complete 36 UOC at level 1")
    }

    #[test]
    fn test_uoc_from() {
        let requirements = Requirements::try_new("Prerequisites: 6 UOC from following course COMM1100 or COMM1120 or COMM1140").unwrap();
        assert_eq!(requirements.to_string(), "Complete 6 UOC from following course [COMM1100, COMM1120, COMM1140]")
        
    }

    #[test]
    fn test_uoc_overall() {
        let requirements = Requirements::try_new("Prerequisites: finish 112 UOC overall").unwrap();
        assert_eq!(requirements.to_string(), "Complete at least 112 UOC")

    }

    #[test]
    fn test_basic_binary() {
        let requirements = Requirements::try_new("Prerequisites: COMP1511 and COMP1521").unwrap();
        assert_eq!(requirements.to_string(), "(COMP1511 and COMP1521)")
    }

    #[test]
    fn test_basic_list() {
        let requirements = Requirements::try_new("Prerequisites: COMP1511, COMP1521").unwrap();
        assert_eq!(requirements.to_string(), "Require all of following conditions: COMP1511, COMP1521")
    }

    #[test]
    fn test_long_binary() {
        let requirements = Requirements::try_new("Prerequisites: COMP1511 and COMP1521 OR COMP1531 OR COMP2521 And COMM1999").unwrap();
        assert_eq!(requirements.to_string(), "(COMP1511 and (COMP1521 or (COMP1531 or (COMP2521 and COMM1999))))")

    }
    #[test]
    fn test_edge_list() {
        let requirements = Requirements::try_new("Prerequisites: COMP1511, and COMM1100").unwrap();
        assert_eq!(requirements.to_string(), "Require all of following conditions: COMP1511, COMM1100")
    }

    #[test]
    fn test_nest_node() {
        let requirements = Requirements::try_new("Prerequisites: (COMP1511 and COMP1521) or (COMP3311 and COMM1999)").unwrap();
        assert_eq!(requirements.to_string(), "((COMP1511 and COMP1521) or (COMP3311 and COMM1999))")
        
    }

    #[test]
    fn test_program_and_course() {
        let requirements = Requirements::try_new("Prerequisites: Must enroll in master of commerce 3784 and complete COMM1110").unwrap();
        assert_eq!(requirements.to_string(), "(3784 and COMM1110)");
        let requirements = Requirements::try_new("Prerequisites: Must enroll in master of commerce 3784, complete COMM1110").unwrap();
        assert_eq!(requirements.to_string(), "Require all of following conditions: 3784, COMM1110");
    }

    #[test]
    fn test_code_and_uoc() {
        let requirements = Requirements::try_new("Prerequisites: Must enroll in master of commerce 3784 and complete 12 uoc from COMM1100 or COMM1120 or COMM1140").unwrap();
        assert_eq!(requirements.to_string(), "(3784 and Complete 12 UOC from following course [COMM1100, COMM1120, COMM1140])")
    }

    #[test]
    fn test_code_and_wam() {
        let requirements = Requirements::try_new("Prerequisites: Must enroll in master of commerce 3784 and wam of 65 or above").unwrap();
        assert_eq!(requirements.to_string(), "(3784 and WAM of at least 65)")
    }

    #[test]
    fn test_wam_and_uoc() {
        let requirements = Requirements::try_new("Prerequisites: WAM of 85 and complete 102 uoc at level 1").unwrap();
        assert_eq!(requirements.to_string(), "(WAM of at least 85 and Complete 102 UOC at level 1)")

    }

    #[test]
    fn test_empty_str() {
        let requirements = Requirements::try_new("").unwrap();
        assert_eq!(requirements.to_string(), "")
    }

    #[test]
    fn test_unused_str() {
        let requirements = Requirements::try_new("Exclusion: MECH3211, MTRN3212").unwrap();
        assert_eq!(requirements.to_string(), "")
    }

    #[test]
    fn test_br() {
        let requirements = Requirements::try_new("Exclusion: MECH3211, MTRN3212<br/>Prerequisites: MATH1231 OR DPST1014 OR MATH1241").unwrap();
        assert_eq!(requirements.to_string(), "(MATH1231 or (DPST1014 or MATH1241))")
    }

    #[test]
    fn test_prefix() {
        let requirements = Requirements::try_new("Pre-requisites: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "COMP1511");
        let requirements = Requirements::try_new("Prerequisite: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "COMP1511");
        let requirements = Requirements::try_new("Pre-requisite: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "COMP1511");
        let requirements = Requirements::try_new("Prerequisites: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "COMP1511");
        let requirements = Requirements::try_new("Co-requisite: COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "");
        let requirements = Requirements::try_new("COMP1511").unwrap();
        assert_eq!(requirements.to_string(), "");

    }

    #[test]
    fn test_text_nested() {
        let requirements = Requirements::try_new("Pre-requisites: (TEXT [Major in COMMMA and something like (this)] and COMM1140) or (TEXT [ Major in COMMMB] and COMM1120)").unwrap();
        assert_eq!(requirements.to_string(), "((Major in COMMMA and something like ( this ) and COMM1140) or (Major in COMMMB and COMM1120))");
    }

    #[test]
    fn test_complex() {
        let requirements = Requirements::try_new("Pre-requisites: (TEXT [ this is a text] and COMM1140 or COMM1190) or (wam of 85 and (complete at least 102 uoc at level 3 or program 3999)), COMM1110").unwrap();
        assert_eq!(requirements.to_string(), "Require all of following conditions: ((this is a text and (COMM1140 or COMM1190)) or (WAM of at least 85 and (Complete 102 UOC at level 3 or 3999))), COMM1110");
    }

    // #[test]
    // fn test_eval() {
    //     let requirements = Requirements::try_new("Pre-requisites: (TEXT [ this is a text] and COMM1140 or COMM1190) or (wam of 85 and (complete at least 102 uoc at level 3 or program 3999)), COMM1110").unwrap();
    // }

}


// Pre-requisites
// Prerequisite
// Pre-requisite
// Prerequisites
// Co-requisite
// aaa or bbb, and ccc
// aaa or bbb or, acc
// faculty UNSW Global 
// labelling until INFS4858 34637 from line 2325

// ----------------------
// NEED TO DO
// 34637 - end of files
// 588 - 2325

