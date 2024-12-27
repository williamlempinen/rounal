//#[derive(Debug, Clone)]
//pub enum Priority {
//    1,
//    2,3,4,5,6,7
//}

#[derive(Debug, Clone)]
pub struct Log {
    pub priority: u8,
    pub timestamp: String,
    pub log_message: String,
    pub hostname: String,
    pub service: Option<String>,
}
