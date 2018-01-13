use std::sync::RwLock;

pub struct ApiData {
    pub config: RwLock<String>,
    pub routes: Vec<String>,
}

pub fn new() -> ApiData {
    ApiData {config: RwLock::new(String::new()), routes: Vec::new()}
}
