use executer::Context;
use tests::model::{Database, Stop};

impl Context for Database {}

graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field Stops() -> Vec<Stop> {
        self.stops
    }
});
