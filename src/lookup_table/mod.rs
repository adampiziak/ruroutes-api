use std::collections::HashMap;

struct Campus {
    tag: String,
    name: String
}

impl Campus {
    fn new(tag: String, name: String) -> Campus {
        Campus { tag, name }
    }
}


pub fn getRouteCampusTable() -> HashMap<String, Vec<String>> {
    let mut campuses = HashMap::new();

    campuses.insert("penn".to_string(), vec!["newark".to_string()]);
    campuses.insert("connect".to_string(), vec!["newark".to_string()]);
    campuses.insert("a".to_string(), vec!["busch".to_string(), "college ave".to_string()]);
    campuses.insert("b".to_string(), vec!["busch".to_string(), "livingston".to_string()]);
    campuses.insert("c".to_string(), vec!["busch".to_string()]);
    campuses.insert("ee".to_string(), vec!["college ave".to_string(), "cook douglass".to_string()]);
    campuses.insert("f".to_string(), vec!["college ave".to_string(), "cook douglass".to_string()]);
    campuses.insert("h".to_string(), vec!["busch".to_string(), "college ave".to_string()]);
    campuses.insert("lx".to_string(), vec!["college ave".to_string(), "livingston".to_string()]);
    campuses.insert("rexl".to_string(), vec!["cook douglass".to_string(), "livingston".to_string()]);
    campuses.insert("rexb".to_string(), vec!["busch".to_string(), "cook douglass".to_string()]);
    campuses.insert("w1".to_string(), vec!["college ave".to_string()]);
    campuses.insert("rbhs".to_string(), vec!["downtown".to_string()]);
    campuses.insert("ccexp".to_string(), vec!["newark".to_string()]);
    campuses.insert("kearney".to_string(), vec!["newark".to_string()]);


    campuses
}

