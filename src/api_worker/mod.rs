use std::{thread, time};
use redis::{Client, Connection, Commands};
use serde_json;
use serde_json::value::Value;
use model::{Route, RouteSchedule};
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
    tag: String
}

#[derive(Serialize, Deserialize)]
struct RouteOrder {
    route_tag: String,
    order: Vec<String>
}

// Called when new routeConfig data is fetched
// Does not perform request, simply assembles long query so data_fetcher can use it
pub fn create_predictions_query() {
    let conn = get_redis_connection();
    let route_config: String = conn.get("config").unwrap();
    let route_config: Config = serde_json::from_str(&route_config).unwrap();
    let mut route_order: HashMap<String, Vec<String>> = HashMap::new();
    let mut query = String::from("");
    for route in route_config.route {
        let route_tag = &route.tag;
        let mut route_stops_query = String::new();
        for stop in route.stop {
            route_order.entry(route_tag.to_string()).or_insert(Vec::new()).push(stop.tag.clone());
            let stop_query = format!("&stops={}|{}", &route_tag, &stop.tag);
            route_stops_query.push_str(&stop_query);
        }
        query.push_str(&route_stops_query);
    }
    let mut order: Vec<RouteOrder> = Vec::new();
    for (key, value) in route_order {
        order.push(RouteOrder {
            route_tag: key,
            order: value,
        });      
    }
    let serialized = serde_json::to_string(&order).unwrap();
    let _: () = conn.set("route_order", serialized).unwrap();
    let _: () = conn.set("multi_stops", query).unwrap();
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
    let conn = get_redis_connection();
    let raw: String = conn.get("raw_predictions").unwrap();
    let mut routes: HashMap<String, Route> = HashMap::new();
    let raw_order: String = conn.get("route_order").unwrap();
    let route_order: Vec<RouteOrder> = serde_json::from_str(&raw_order).unwrap();

    let data: Value = serde_json::from_str(&raw).unwrap();
    let predictions = &data["predictions"];
    let len = predictions.as_array().unwrap().len();
    for i in 0..len {
        let route_tag_json = &predictions[i]["routeTag"];
        let schedule_json = &predictions[i]["direction"]["prediction"];
        if route_tag_json.is_null() || schedule_json.is_null() {
            continue;
        }

        let route_tag = route_tag_json.as_str().unwrap().to_string();
        let mut old_stops = Vec::new();
        let mut old_schedule = Vec::new();
        if routes.contains_key(&route_tag.clone()) {
            match routes.get(&route_tag) {
                Some(ref mut old_route) => {
                    old_stops = old_route.stops.clone();
                    old_schedule = old_route.schedule.clone();
                },
                _ => println!("Not found!"),
            }
        }
        
        old_stops.insert(0, predictions[i]["stopTag"].as_str().unwrap().to_string());
        let mut stop_times: Vec<u64> = Vec::new();
        if schedule_json.is_array() {
            let pred_len = schedule_json.as_array().expect("expected array").len();
            for i in 0..pred_len {
                if schedule_json[i]["epochTime"].is_string() {
                    stop_times.push(schedule_json[i]["epochTime"].as_str().unwrap().parse::<u64>().unwrap());
                }
            }
        }
       let next_stop = RouteSchedule {
           stop_tag: predictions[i]["stopTag"].as_str().unwrap().to_string(),
           stop_title: predictions[i]["stopTitle"].as_str().unwrap().to_string(),
           times: stop_times
        };
        
        old_schedule.push(next_stop);
        
        let route = Route {
            name: route_tag.clone(),
            stops: old_stops.clone(),
            schedule: old_schedule.clone()
        };


        routes.insert(route_tag, route);
    }

    let mut all_routes = Vec::new();

    for (_key, mut route) in routes {
        let ro = route_order.iter().find(|ref x| x.route_tag == route.name);
        match ro {
            None => println!("not found!"),
            Some(n) => {
                let mut new_order: Vec<RouteSchedule> = Vec::new();
                for o in n.order.clone() {
                    match route.schedule.iter().find(|a| a.stop_tag == o.to_string()) {
                        Some(route_schedule) => new_order.push(route_schedule.clone()),
                        None => continue,
                    };
                }
                route.schedule.clear();
                route.schedule.append(&mut new_order);
            },
        }
//        println!("route ({}) found.", o.name);
        all_routes.push(route);
    }

    let serialized = serde_json::to_string(&all_routes).unwrap();

    let _: () = conn.set("predictions", serialized).unwrap();
}
