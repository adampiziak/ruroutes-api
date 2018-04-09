use std::collections::HashMap;
use redis::{Client, Connection};
use serde_json;
use std::mem::{drop};




#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    name: String,
    tag: String,
    #[serde(default="default_false")]
    active: bool,
    #[serde(default="default_empty")]
    stops: Vec<Stop>,
    predictions: Vec<Prediction>
}

impl Route {
    pub fn create_routes_from_pred(pred_json: String) -> Vec<Route> {
        let data: serde_json::Value = serde_json::from_str(&pred_json).unwrap(); 
        
        Vec::new()
    }

    pub fn new() -> Route {
        let route = Route {
            name: String::new(),
            tag: String::new(),
            active: false,
            stops: Vec::new(),
            predictions: Vec::new(),
        };

        route
    }

}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stop {
    name: String,
    tag: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prediction {
    stop_tag: String,
    arrival_times: Vec<u64>
}


fn create_routes(raw: RawPrediction) {
    let mut routes = HashMap::new();

    for p in raw.predictions {
        let route = Route {
            name: p.routeTitle.clone(),
            tag: p.routeTag.clone(),
            active: p.direction.prediction.len() > 0,
            stops: Vec::new(),
            predictions: Vec::new(),
        };

        routes.entry(route.tag.clone()).or_insert(route);
        let mut r = Route::new();
        let mut new_predictions = Vec::new();
        let mut new_stops = Vec::new();
        let mut new_active = false;
        {
            let r = &routes.get(&p.routeTag.clone()).unwrap();
            
            new_stops = r.stops.clone();
            new_stops.push(Stop { name: p.stopTitle, tag: p.stopTag.clone() });

            new_predictions = r.predictions.clone();
            new_active = r.active;
        }

        let mut arrival_times = Vec::new();

        for time in p.direction.prediction {
            arrival_times.push(time.epochTime.parse::<u64>().unwrap());
        }
        
        let mut new_p = Prediction {
            stop_tag: p.stopTag.clone(),
            arrival_times
        };

        new_predictions.push(new_p);
        

        let new_route = Route {
            stops: new_stops,
            predictions: new_predictions,
            tag: p.routeTag.clone(),
            name: p.routeTitle.clone(),
            active: new_active,
        };

        drop(r);
        let tag = p.routeTag.clone();
        routes.insert(tag, new_route);
    }

    println!("Printing keys...");
    for (key, value) in &routes {
        println!("{}", key);
    }

    

    
}





#[derive(Deserialize)]
struct RawPrediction {
    predictions: Vec<RawPredictionRouteStop>
}

#[derive(Deserialize)]
struct RawPredictionRouteStop {
    stopTag: String,
    stopTitle: String,
    routeTag: String,
    routeTitle: String,
//    #[serde(default="default_empty_pred")]
    direction: RawPredictionDirection
}

#[derive(Deserialize)]
struct RawPredictionDirection {
//    #[serde(deserialize_with = "deserialize_predictions")]
    prediction: Vec<RawPredictionBus>
}

#[derive(Deserialize)]
struct RawPredictionBus {
    epochTime: String
}

fn get_redis_connection() -> Connection {
    let conn = Client::open("redis://localhost:6379")
        .unwrap()
        .get_connection()
        .unwrap();
    conn
}

fn default_false() -> bool {
    false
}

fn default_empty() -> Vec<Stop> {
    Vec::new()
}

fn default_empty_pred() -> RawPredictionDirection {
    RawPredictionDirection { prediction: Vec::new() }
}
