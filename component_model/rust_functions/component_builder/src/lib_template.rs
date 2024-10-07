#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn func_wrapper(json_string: std::string::String) -> std::string::String {
        let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let result = func(json).unwrap();
        // println!("From WASM:\n\tResult: {}", result);
        result.to_string()
    }
}

bindings::export!(Component with_types_in bindings);


// The function that will be called by the wrapper will be added bellow
