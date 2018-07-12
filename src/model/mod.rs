#[derive(Serialize, Debug)]
pub struct Route {
    pub tag: String,
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

impl RouteSchedule {
    pub fn new(stop_title: String, stop_tag: String) -> RouteSchedule {
        RouteSchedule {
            stop_title,
            stop_tag,
            times: Vec::new()
        }
    }
}

impl Route {
    pub fn new(name: String, tag: String) -> Route {
        Route {
            tag,
            name,
            stops: Vec::new(),
            schedule: Vec::new()
        }
    }
}

impl Stop {
    pub fn new(name: String, tag: String, location: Location) -> Stop {
        Stop {
            name,
            tag,
            campus: String::from("other"),
            schedule: Vec::new(),
            location,
        }
    }
}

impl StopSchedule {
    pub fn new(route: String) -> StopSchedule {
        StopSchedule {
            route,
            times: Vec::new(),
        }
    }
}
impl Clone for RouteSchedule {
    fn clone(&self) -> Self {
        RouteSchedule {stop_tag: self.stop_tag.clone(), stop_title: self.stop_title.clone(), times: self.times.clone()}
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route {tag: self.tag.clone(), name: self.name.clone(), stops: self.stops.clone(), schedule: self.schedule.clone()}
    }
}

impl Clone for StopSchedule {
    fn clone(&self) -> Self {
        StopSchedule { route: self.route.clone(), times: self.times.clone() }
    }
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location(self.0.clone(), self.1.clone())
    }
}

#[derive(Serialize, Debug)]
pub struct Stop {
    pub name: String,
    pub tag: String,
    pub campus: String,
    pub schedule: Vec<StopSchedule>,
    pub location: Location
}

#[derive(Serialize, Debug)]
pub struct StopSchedule {
    pub route: String, // Name, not tag
    pub times: Vec<u64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Location(pub f64, pub f64);

