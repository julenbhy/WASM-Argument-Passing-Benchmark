use wasmtime::*;
use wasi_common::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    
    let args: Vec<String> = std::env::args().collect();

    // Version 1: Take multiple arguments and create a JSON object
    //let input = parse_args(args.clone());
    // Version 2: Take a single argument (already a JSON object)
    let input = args[2].clone();
    println!("Input: {:?}", input);

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);

    // Load the module from disk
    let bytes = std::fs::read(&args[1]).unwrap();
    let module = unsafe { Module::deserialize(&engine, bytes)? };

    //Instantiate the module
    let instance_pre = linker.instantiate_pre(&module)?;
    let instance = instance_pre.instantiate(&mut store).unwrap();

    // Pass the input to the WASM module
    let Ok(set_input) = instance.get_typed_func::<u32, u32>(&mut store, "set_input") else {
        anyhow::bail!("Failed to get set_input");
    };
    // Allocate wASM memory for the input
    let input_ptr = set_input.call(&mut store, input.len() as u32)? as usize;
    // Print the pointer to the input
    println!("Input pointer: {}", input_ptr);
    
    let Some(memory) = instance.get_memory(&mut store, "memory") else {
        anyhow::bail!("Failed to get WASM memory");
    };

    let input_bytes = input.as_bytes();
    memory.data_mut(&mut store)[input_ptr..(input_ptr + input_bytes.len())].copy_from_slice(input_bytes);

    // Call the _start function
    let func = instance.get_typed_func::<(), ()>(&mut store, "_start").unwrap();
    func.call(&mut store, ())?;

    // Get the result from the WASM module execution
    let Ok(get_result_len) = instance.get_typed_func::<(), u32>(&mut store, "get_result_len") else {
        anyhow::bail!("Failed to get get_result_len");
    };

    let Ok(get_result) = instance.get_typed_func::<(), u32>(&mut store, "get_result") else {
        anyhow::bail!("Failed to get get_result");
    };

    let length = get_result_len.call(&mut store, ())? as usize;
    let content_ptr = get_result.call(&mut store, ())? as usize;

    let content = memory.data(&store)[content_ptr..(content_ptr + length)].to_vec();

    let result_string = String::from_utf8(content)?;

    println!("From rust:\n\tResult ptr: {}\n\tResut length: {}\n\tResult {}", content_ptr, length, result_string);

    Ok(())
}


#[allow(dead_code)]
fn parse_args(args: Vec<String>) -> String {
    let mut input = String::from("{");
    for (i, arg) in args.iter().skip(2).enumerate() {
        let param_string = format!("\"param{}\":{}", i + 1, arg);
        if i < args.len() - 3 {
            input.push_str(&format!("{},", param_string));
        } else {
            input.push_str(&param_string);
        }
    }
    input.push('}');
    input
}