use std::{thread, time};
use redis::{Client, Connection, Commands};
use serde_json;
use serde_json::value::Value;
use model::{Route, RouteSchedule, Stop, StopSchedule, Location};
use std::collections::HashMap;

pub fn start() {
    thread::spawn(|| {
        //let conn = get_redis_connection();
        loop {

            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}

fn get_redis_connection() -> Connection {
    let conn = Client::open("redis://localhost:6379")
        .unwrap()
        .get_connection()
        .unwrap();
    conn
}

#[derive(Serialize, Deserialize)]
struct Config {
    route: Vec<ConfigRoute>
}

#[derive(Serialize, Deserialize)]
struct ConfigRoute {
    tag: String,
    stop: Vec<ConfigStop>
}

#[derive(Serialize, Deserialize)]
struct ConfigStop {
    tag: String,
    lon: String,
    lat: String,
}

#[derive(Serialize, Deserialize)]
struct RouteOrder {
    route_tag: String,
    order: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct StopLocation {
    tag: String,
    location: Location
}

// Called when new routeConfig data is fetched
// Does not perform request, simply assembles long query so data_fetcher can use it
pub fn create_predictions_query() {
    let conn = get_redis_connection();
    let route_config: String = conn.get("config").expect("api_worker->create_predictions_query: unable to retrieve config from redis");
    let route_config: Config = serde_json::from_str(&route_config).unwrap();
    
    let mut query = String::new();
    for route in route_config.route {
        let route_tag = &route.tag;
        let mut route_stops_query = String::new();
        for stop in route.stop {
            let stop_query = format!("&stops={}|{}", &route_tag, &stop.tag);
            route_stops_query.push_str(&stop_query);
        }
        query.push_str(&route_stops_query);
    }

    let _: () = conn.set("schedule_query", query).unwrap();
}

pub fn store_route_order() {
    let conn = get_redis_connection();
    let route_config: String = conn.get("config").expect("api_worker->create_predictions_query: unable to retrieve config from redis");
    let route_config: Config = serde_json::from_str(&route_config).unwrap();
    
    let mut route_orders: Vec<RouteOrder> = Vec::new();
    for route in route_config.route {
        let mut o = RouteOrder {
            route_tag: route.tag.clone(),
            order: Vec::new(),
        };
        
        for stop in route.stop {
            o.order.push(format!("{}", &stop.tag));
        }
        route_orders.push(o);
    }
    
    let serialized = serde_json::to_string(&route_orders).unwrap();
    let _: () = conn.set("route_order", serialized).unwrap();
    
}

pub fn store_stop_locations() {
    let conn = get_redis_connection();
    let route_config: String = conn.get("config").expect("api_worker->create_predictions_query: unable to retrieve config from redis");
    let route_config: Config = serde_json::from_str(&route_config).unwrap();

    let mut stops: HashMap<String, Location> = HashMap::new();
    for route in route_config.route {
        for stop in route.stop {
            let lon = stop.lon.parse::<f64>().expect("stop has no longitude");
            let lat = stop.lat.parse::<f64>().expect("stop has no latitude");
            stops.entry(stop.tag.clone()).or_insert(Location(lon, lat));
        }
    }
    let mut populated_stops = Vec::new();
    for (key, val) in stops {
        populated_stops.push(StopLocation {
            tag: key,
            location: val
        });
    }
    
    let serialized = serde_json::to_string(&populated_stops).unwrap();
    let _: () = conn.set("stop_locations", serialized).unwrap();
    
}

#[derive(Serialize, Deserialize)]
struct Pred {
    predictions: Vec<PredictionsStop>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PredictionsStop {
    stop_tag: String,
    route_tag: String,
    #[serde(default="default_predictions")]
    direction: PredictionsDirection,
}

#[derive(Serialize, Deserialize)]
struct PredictionsDirection {
    prediction: Vec<PredictionsBus>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PredictionsBus {
    epoch_time: String
}

fn default_predictions() -> PredictionsDirection {
    PredictionsDirection { prediction: Vec::new() }
}

pub fn process_raw_predictions() {
    // Retrieve data from redis
    let conn = get_redis_connection();
    let raw_predictions: String = conn.get("raw_predictions").unwrap();
    let stop_locations_json: String = conn.get("stop_locations").unwrap();
    let raw_order: String = conn.get("route_order").unwrap();

    // Parse json
    let data: Value = serde_json::from_str(&raw_predictions).unwrap();
    let stop_locations: Vec<StopLocation> = serde_json::from_str(&stop_locations_json).unwrap();
    let route_order: Vec<RouteOrder> = serde_json::from_str(&raw_order).unwrap();

    let predictions = &data["predictions"];

    let route_predictions = parse_routes(predictions, route_order);
    let stop_predictions = parse_stops(predictions, stop_locations);
    let _: () = conn.set("route_predictions", route_predictions).unwrap();
    let _: () = conn.set("stop_predictions", stop_predictions).unwrap();
    // Iterate through each prediction (route, stop) and group by route
    
}
fn get_campus(location: (f64, f64)) -> String {
    let newark      = (-74.153312, -74.203008, 40.718745, 40.757116);
    let busch       = (-74.451289, -74.469602, 40.511145, 40.529804);
    let livingston  = (-74.428853, -74.443085, 40.157373, 40.531024);
    let college_ave = (-74.443736, -74.463754, 40.497404, 40.506850);
    let cook_doug   = (-74.429729, -74.437711, 40.475680, 40.480068);
    if is_within(location, newark) {
        return String::from("Newark");
    }
    if is_within(location, busch) {
        return String::from("Busch");
    }
    if is_within(location, livingston) {
        return String::from("Livingston");
    }
    if is_within(location, college_ave) {
        return String::from("College Ave");
    }
    if is_within(location, cook_doug) {
        return String::from("Cook Douglas");
    }
    String::from("other")
}

fn is_within(point: (f64, f64), region: (f64, f64, f64, f64)) -> bool {
    point.0 < region.0 && point.0 > region.1 && point.1 > region.2 && point.1 < region.3
}

fn parse_stops(predictions: &serde_json::Value, stop_locations: Vec<StopLocation>) -> String {
    let mut stops: HashMap<String, Stop> = HashMap::new();
    let predictions_size = predictions.as_array().unwrap().len();
    for i in 0..predictions_size {
        let schedule_json = &predictions[i]["direction"]["prediction"];
        if schedule_json.is_null()  {
            continue;
        }
        
        let route_name = predictions[i]["routeTitle"].as_str().unwrap().to_string();
        let stop_name  = predictions[i]["stopTitle"].as_str().unwrap().to_string();
        let stop_tag   = predictions[i]["stopTag"].as_str().unwrap().to_string();

        let mut route = StopSchedule::new(route_name);
        let stop_location = stop_locations.iter().find(|&x| x.tag == stop_tag).expect("stop not found!");
        let mut stop  = stops.entry(stop_tag.clone()).or_insert(Stop::new(stop_name, stop_tag,  stop_location.location.clone()));
        stop.campus = get_campus((stop.location.0, stop.location.1));
        let mut route_schedule = Vec::new();

        if schedule_json.is_array() {
            let schedule_size = schedule_json.as_array().unwrap().len();
            for i in 0..schedule_size {
                let time = &schedule_json[i]["epochTime"];
                if time.is_string() {
                    let eta = time.as_str().unwrap().parse::<u64>().unwrap();
                    route_schedule.push(eta);
                }
            }            
        }  else if schedule_json.is_object() {
            let time = &schedule_json["epochTime"];
            if time.is_string() {
                let eta = time.as_str().unwrap().parse::<u64>().unwrap();
                route_schedule.push(eta);                
            }
        }

        route.times = route_schedule;
        stop.schedule.push(route);
    }

    let mut all_stops = Vec::new();
    for (_, value) in stops {
        all_stops.push(value);
    }

    // Sort by stop name
    all_stops.sort_by(|a, b| a.name.cmp(&b.name));
    serde_json::to_string(&all_stops).unwrap()
}


fn parse_routes(predictions: &serde_json::Value, route_order: Vec<RouteOrder>) -> String{
    let mut routes: HashMap<String, Route> = HashMap::new();
    let predictions_size = predictions.as_array().unwrap().len();
    for i in 0..predictions_size {
        // Filter out inactive routes
        let schedule_json = &predictions[i]["direction"]["prediction"];
        if schedule_json.is_null()  {
            continue;
        }
        
        let route_name = predictions[i]["routeTitle"].as_str().unwrap().to_string();
        let route_tag  = predictions[i]["routeTag"].as_str().unwrap().to_string();
        let stop_name  = predictions[i]["stopTitle"].as_str().unwrap().to_string();
        let stop_tag   = predictions[i]["stopTag"].as_str().unwrap().to_string();

        let mut stop = RouteSchedule::new(stop_name, stop_tag);
        let mut route = routes.entry(route_tag.clone()).or_insert(Route::new(route_name, route_tag.clone()));

        let mut stop_schedule = Vec::new();

        if schedule_json.is_array() {
            let schedule_size = schedule_json.as_array().unwrap().len();
            for i in 0..schedule_size {
                let time = &schedule_json[i]["epochTime"];
                if time.is_string() {
                    let eta = time.as_str().unwrap().parse::<u64>().unwrap();
                    stop_schedule.push(eta);
                }
            }            
        }  else if schedule_json.is_object() {
            let time = &schedule_json["epochTime"];
            if time.is_string() {
                let eta = time.as_str().unwrap().parse::<u64>().unwrap();
                stop_schedule.push(eta);                
            }
        }

        stop.times = stop_schedule;
        route.schedule.push(stop);
    }

    let mut all_routes: Vec<Route> = Vec::new();

    for route in &route_order {
        if !routes.contains_key(&route.route_tag) {
            continue;
        }
        let unordered_route = routes.get(&route.route_tag).expect("unable to find route in hashmap.");
        let mut ordered_route: Route = unordered_route.clone();
        ordered_route.schedule.clear();
        for stop in &route.order {
            let target_stop = match unordered_route.schedule.iter().find(|x| x.stop_tag == stop.to_string()) {
                Some(x) => x,
                None    => {
                    // stop inactive
                    continue;
                }
            };
            ordered_route.schedule.push(target_stop.clone());
        }

        all_routes.push(ordered_route);
    }

    serde_json::to_string(&all_routes).unwrap()

}
