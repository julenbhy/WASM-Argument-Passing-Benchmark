use anyhow::{Result, anyhow};

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

pub fn func(json: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
    // Get the input values from the JSON
    let a = json["param1"].as_i64().ok_or(anyhow!("'param1' not found in JSON"))? as i32;

    // Calculate the Fibonacci number
    let result = fibonacci(a);

    Ok(serde_json::json!({"result": result}))
}