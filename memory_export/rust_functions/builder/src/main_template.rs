use serde_json::Value;
use std::ptr;
use std::alloc::{alloc, Layout};


static mut INPUT: *mut u8 = ptr::null_mut();
static mut INPUT_LEN: usize = 0;

static mut RESULT: Option<String> = None;

fn main() -> Result<()> {
    unsafe {
        // Parse the input JSON
        let input_slice = std::slice::from_raw_parts(INPUT, INPUT_LEN);
        let input_str = std::str::from_utf8(input_slice).unwrap();
        let json: Value = serde_json::from_str(input_str).unwrap();

        // Call the function
        let result_json = func(json)?;

        // Save the result as a string
        RESULT = Some(result_json.to_string());

        /*
        println!(
            "From rust:\n\tResult ptr (decimal): {}\n\tResult length: {}\n\tResult: {}",
            RESULT.as_ref().unwrap().as_ptr() as usize,
            RESULT.as_ref().unwrap().len(),
            RESULT.as_ref().unwrap()
        );
        */
    }

    Ok(())
}

// Function that allocates memory for the input
#[no_mangle]
pub extern "C" fn set_input(size: usize) -> *mut u8 {
    unsafe {
        INPUT = alloc(Layout::from_size_align(size, 1).unwrap());
        INPUT_LEN = size;
        INPUT
    }
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
