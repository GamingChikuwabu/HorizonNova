use std::collections::HashMap;

use log::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
struct TestSerializer<'a>
{
    name: String,
    age: &'a u32,
    map : HashMap<String,i32>,
}


fn main() {
    let age = 10;

    let mut test = TestSerializer{
        name: "test".to_string(),
        age: &age,
        map: HashMap::new(),
    };

    test.map.insert("test".to_string(), 10);
    test.map.insert("test2".to_string(), 20);

    let serialized = serde_json::to_string(&test).unwrap();

    debug_log!("serialized: {}", serialized);
}