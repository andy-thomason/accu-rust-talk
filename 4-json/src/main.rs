

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Query {
    how_many: i32,
    what: String,
}

fn main() {
    let query = Query { how_many: 10, what: "widgets".to_string() };

    println!("query={:?}", &query);

    let query_as_json = serde_json::to_string(&query).unwrap();

    println!("query_as_json={}", &query_as_json);

    let query : Query = serde_json::from_str(query_as_json.as_str()).unwrap();

    println!("query={:?}", &query);
}
