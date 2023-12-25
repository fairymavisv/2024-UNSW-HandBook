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