use core::num;

use rayon::iter;

use crate::{course::Course, utlis::{CourseCode, ProgramCode}};

#[derive(Debug)]
pub struct Requirements {
    raw_requirements: String,

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

impl Requirements {
    pub fn new(raw_requirements: String) -> Requirements {
        Requirements {
            raw_requirements,
        }
    }

    fn clean(raw_requirements: &str) -> String {
        let cleaned_lines = raw_requirements.trim().replace("[", " [ ")
            .replace("]", " ] ")
            .replace("(", " ( ")
            .replace(")", " ) ")
            .replace(",", " , ")
            .replace(".", " ")
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

    pub fn tokenize(raw_requirements: &str) -> Vec<Token> {
        let cleanned_requirements = Requirements::clean(raw_requirements);
        let mut tokens: Vec<Token> = Vec::new();
        let mut itr = cleanned_requirements.split_ascii_whitespace();
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
                "UOC" | "uoc" | "Uoc" => {
                    tokens.push(Token::KEYWORD(Keyword::UOC));
                },
                "WAM" | "wam" => {
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


    pub fn parse(mut tokens: Vec<Token>) -> Option<Requirements> {
        // let mut tokens = Requirements::tokenize(raw_requirements);
        if tokens.len() == 0 {
            return None;
        }
        let mut buffer: Vec<Box<dyn Node>> = Vec::new();
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
                Token::NUMBER(number) => {
                    println!("{}", number);
                    let next = tokens.pop();
                    if next.is_none() {
                        break;
                    }
                    match next.unwrap() {
                        Token::KEYWORD(keyword) => {
                            match keyword {
                                Keyword::UOC => { 
                                    let position = tokens.last();
                                    if position.is_none() {
                                        continue;
                                    }
                                    match position.unwrap() {
                                        Token::PREPOSITION(preposition) => {
                                            match preposition {
                                                Preposition::FROM => {
                                                    let mut node = Box::new(UOCFromNode::new(number ));
                                                    if node.parse(&mut tokens) {
                                                        buffer.push(node);

                                                    }
                                                },
                                                Preposition::AT => {
                                                    let mut node = Box::new(UOCAtLevelNode::new(number ));
                                                    if node.parse(&mut tokens) {
                                                        buffer.push(node);
                                                    }
                                                },
                                                _ => ()
                                            }
                                        },
                                        _ => {
                                            buffer.push(Box::new(UOCNode::new(number)));
                                        }
                                    }

                                },
                                Keyword::WAM => {
                                    buffer.push(Box::new(WamNode::new(number)));
                                },
                                _ => ()
                                
                            }
                        },
                        
                        _ => {
                            println!("Other");
                            dbg!(tokens);
                            return None;
                        }
                    }

                },
                Token::COMMA => {
                    // TODO 
                    // Parse buffer into binary node
                    println!(",");
                },
                Token::BRACKET(bracket) => {
                    println!("{:?}", bracket);
                    // TODO
                    // Recursively parse nodes in tokens

                },
                Token::KEYWORD(keyword) => {
                    match keyword {
                        Keyword::WAM => {
                            let mut node = Box::new(WamNode::new(0));
                            if node.parse(&mut tokens) {
                                buffer.push(node);
                            }
                        },
                        _ => ()
                    }
                    println!("{:?}", keyword);
                },
                _  => {
                    println!("Other");
                    dbg!(tokens);
                    return None;
                }
            }
        }

        todo!()
        // Requirements {

        // }
    }
}

pub trait Node {
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool;

}

// TODO 
// BABS2204 OR BABS2264 OR BIOC2201
// (BABS2204 OR BABS2264) OR BIOC2201
// OR OR BABS2204 BABS2264  BIOC2201

struct BinaryNode {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
    operator: Operator,
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

    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        tokens.pop();
        let token = tokens.pop();
        if token.is_none() {
            return false;
        }
        match token.unwrap() {
            Token::KEYWORD(keyword) => {
                match keyword {
                    Keyword::LEVEL => {
                        let token = tokens.pop();
                        if token.is_none() {
                            return false;
                        }


                        match token.unwrap() {
                            Token::NUMBER(level) => {
                                self.level = level;
                                return true;
                            },
                            _ => return false,
                        }
                    },
                    _ => return false,
                }
            },
            _ => return false,
        }
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
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        tokens.pop();
        loop {
            let token = tokens.pop();
            if token.is_none() {
                return false;
            }
            match token.unwrap() {
                Token::CODE(code) => {
                    match code {
                        Code::COURSE(course_code) => {
                            self.code.push(course_code);
                        },
                        _ => return false,
                        
                    }
                },
                _ => (),
            }
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
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        true
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
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        let last = tokens.last();
        if last.is_none() {
            return false;
        }
        if let Token::PREPOSITION(Preposition::OF) = last.unwrap() {
            tokens.pop();
        }
        let wam = tokens.pop();
        if wam.is_none() {
            return false;
        }
        if let Token::NUMBER(num) = wam.unwrap() {
            self.wam = num;
        }
        true
    
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
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        true
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
    fn parse(&mut self, tokens: &mut Vec<Token>) -> bool {
        true
    }
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