use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let input = r#"{"ADDR_CITY":"anaheim","ADDR_STATE_CODE":"CA","ADDR_STREET1":"1 disney way","ADDR_STREET2":"","ADDR_ZIP":"94802","BIRTH_DATE":"1975-03-11"}"#;

    // Parse the input JSON into a Value
    let data: Value = serde_json::from_str(input).expect("Failed to parse JSON");

    // Check if the parsed JSON is an object
    if let Value::Object(map) = data {
        // Convert keys to lowercase
        let lowercase_data: HashMap<String, Value> = map
            .into_iter()
            .map(|(key, value)| (key.to_lowercase(), value))
            .collect();

        // Serialize the modified map back to JSON
        let output = serde_json::to_string(&lowercase_data).expect("Failed to serialize JSON");

        // Print the resulting JSON
        println!("{}", output);
    } else {
        println!("Input JSON is not an object");
    }
}
