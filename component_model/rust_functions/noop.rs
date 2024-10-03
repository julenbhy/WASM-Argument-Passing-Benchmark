use anyhow::anyhow;


pub fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {

    let result = json;

    Ok(serde_json::json!({"result": result}))
}