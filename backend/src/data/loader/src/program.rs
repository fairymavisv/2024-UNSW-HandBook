use crate::utlis::{ProgramCode, StudyLevel};

pub struct Program {
    title: String,
    code: ProgramCode,
    uoc: u8,
    level: u8,
    study_level: StudyLevel,
    components: Vec<Box<dyn ProgramComponent>>,
}
pub trait ProgramComponent {
    
    
}

pub struct CoreComponent {

}
impl ProgramComponent for CoreComponent {
    
}
pub struct StreamComponent {

}
impl ProgramComponent for StreamComponent {
    
}
pub struct ElectiveComponent {

}

impl ProgramComponent for ElectiveComponent {
    
}
pub struct SelectionComponent {

}

impl ProgramComponent for SelectionComponent {
    
}