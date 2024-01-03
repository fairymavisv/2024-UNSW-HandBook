use core::num;
use std::fmt::Display;

use rayon::iter;
use wasm_bindgen::convert::OptionIntoWasmAbi;

use crate::{course::Course, utlis::{CourseCode, ProgramCode}};

// #[derive(Debug)]
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

// #[derive(Debug)]
// pub enum RequirementContent {
//     Binary(BinaryNode),
//     Singel(Box<dyn Node>)
// }

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
            extract_prerequisite!(trimed, "Prerequisites:");
            extract_prerequisite!(trimed, "Pre-requisite:");
            extract_prerequisite!(trimed, "Pre-Prerequisite:");
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
        tokens
    }



    fn parse(tokens: &mut Vec<Token>) -> Result<Option<Box<dyn Node + Send>>, String> {
        // let mut tokens = Requirements::tokenize(raw_requirements);
        // let requirement = Requirements::new(String::from("raw_requirements"));
        if tokens.len() == 0 {
            return Ok(None);
        }
        let mut buffer: Vec<Box<dyn Node + Send>> = Vec::new();
        let mut parsed_node: Vec<Box<dyn Node + Send>> = Vec::new();
        loop {
            tokens.reverse();
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
                    println!("{:?}", operator);
                    if buffer.len() == 1 {
                        let left = buffer.pop().unwrap();
                        let right = Requirements::parse(tokens)?;
                        if right.is_none() {
                            buffer.push(left);
                        } else {
                            let binary = BinaryNode::new(left, right.unwrap(), operator);
                            buffer.push(Box::new(binary));
                        }
                    } else {
                        return Err(String::from("ERROR (BIN-1): Binary operator (e.g and, or) without left hand side, or more than one left hand side token."));
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
                                            match preposition {
                                                Preposition::FROM => {
                                                    let mut node = Box::new(UOCFromNode::new(number ));
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
                        return Err(String::from("ERROR (COM-1): There are more than one node in the buffer."));
                    }                     
                    println!(",");
                },
                Token::BRACKET(bracket) => {
                    println!("{:?}", bracket);
                    let mut num_bracket = 1;
                    let mut sub_token: Vec<Token> = Vec::new();
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
        format!("Require all of following requirement: {}", self.nodes.iter().map(|node| node.get()).collect::<Vec<String>>().join(", "))
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
        loop {
            let token = tokens.pop();
            if token.is_none() {
                return Err(String::from("ERROR (UOC-3A): UOC from without following course code"));
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
        Ok(())     
    }

    fn get(&self) -> String {
        format!("Complete {} UOC from following course [{}]", self.uoc, self.code.iter().map(|code| code.to_string()).collect::<Vec<String>>().join(", "))
    
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
                format!("Course {}", course_code.to_string())
            },
            Code::PROGRAM(program_code) => {
                format!("Program {}", program_code.to_string())
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

    // Add more tests for other nodes and methods here
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

