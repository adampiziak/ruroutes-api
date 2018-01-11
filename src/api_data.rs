pub struct ApiData {
    pub config: String,
    routes: Vec<String>,
}


pub fn new() -> ApiData {
    ApiData {config: String::from("hey there"), routes: Vec::new()}
}
