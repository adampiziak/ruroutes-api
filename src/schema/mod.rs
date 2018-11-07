use model::Stop;

pub struct Database {
    stops: Vec<Stop>
}

juniper::graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema",

    field stops() -> &Vec<Stop> {
        &self.stops
    }
});

juniper::graphql_object!(Stop: Database as "Stops" |&self| {
    description: "A stop",

    field tag() -> String as "The id of the stop" {
        
    }
});
