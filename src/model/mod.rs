#[derive(Serialize, Debug)]
pub struct Route {
    pub name: String,
    pub stops: Vec<String>, 
    pub schedule: Vec<RouteSchedule>
}

#[derive(Serialize, Debug)]
pub struct RouteSchedule {
    pub stop_title: String, // Name, not tag
    pub stop_tag: String,
    pub times: Vec<u64>
}

impl Clone for RouteSchedule {
    fn clone(&self) -> Self {
        RouteSchedule {stop_tag: self.stop_tag.clone(), stop_title: self.stop_title.clone(), times: self.times.clone()}
    }
}

/*
pub struct Stop {
    name: String,
    routes: Vec<String>,
    schedule: Vec<StopSchedule>
}

pub struct StopSchedule {
    route: String, // Name, not tag
    times: Vec<u64>
}
 */

