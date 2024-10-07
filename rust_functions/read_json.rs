
use anyhow::{Result, anyhow};

fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    
    // Get the length, the size (bytes) and the num of elems of the input json
    let len = json.to_string().len();
    let size = std::mem::size_of_val(&json);
    let num_elems = json.as_object().unwrap().len();

    // Build the result string
    let result = format!("Length: {}, Size: {}B, Num Elems: {}", len, size, num_elems);

    Ok(serde_json::json!({"result": result}))
}
