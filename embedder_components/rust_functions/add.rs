use anyhow::anyhow;

pub fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    // Get the input values from the JSON
    let a = json["param1"].as_i64().ok_or(anyhow!("'param1' not found in JSON"))? as i32;
    let b = json["param2"].as_i64().ok_or(anyhow!("'param2' not found in JSON"))? as i32;

    let result = a + b;

    Ok(serde_json::json!({"result": result}))
}