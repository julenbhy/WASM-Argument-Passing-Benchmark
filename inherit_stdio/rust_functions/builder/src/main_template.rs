use serde_json::Value;
use std::ptr;

static mut RESULT: Option<String> = None;

fn main() -> Result<()> {
    // Get args from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json>", args[0]);
        std::process::exit(1);
    }

    // The function receives a JSON string and returns a JSON string
    let json_str = &args[1];
    let json: Value = serde_json::from_str(json_str)?;

    let result_json = func(json)?;

    unsafe {
        RESULT = Some(result_json.to_string());
    }

    /*
    unsafe {
        println!(
            "From rust:\n\tResult ptr (decimal): {}\n\tResult length: {}\n\tResult: {}",
            RESULT.as_ref().unwrap().as_ptr() as usize,
            RESULT.as_ref().unwrap().len(),
            RESULT.as_ref().unwrap()
        );
    }
    */
    Ok(())
}


#[no_mangle]
pub extern "C" fn get_result() -> *const u8 {
    unsafe {
            RESULT.as_ref().unwrap().as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_result_len() -> usize {
    unsafe {
            RESULT.as_ref().unwrap().len()
    }
}
