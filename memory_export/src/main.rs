use wasmtime::*;
use wasi_common::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    
    let args: Vec<String> = std::env::args().collect();

    // Version 1: Take multiple arguments and create a JSON object
    //let input = parse_args(args.clone());
    // Version 2: Take a single argument (already a JSON object)
    let input = args[2].clone();
    //println!("Input: {:?}", input);

    // Time the entire process
    let global_start = std::time::Instant::now();

    // Set up the WASI environment
    let setup_start = std::time::Instant::now();    
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);
    let setup_time = setup_start.elapsed().as_nanos();

    // Load the module from disk
    let load_start = std::time::Instant::now();
    let bytes = std::fs::read(&args[1]).unwrap();
    let module = unsafe { Module::deserialize(&engine, bytes)? };
    let load_time = load_start.elapsed().as_nanos();

    //Instantiate the module
    let instantiation_start = std::time::Instant::now();
    let instance_pre = linker.instantiate_pre(&module)?;
    let instance = instance_pre.instantiate(&mut store).unwrap();
    let instantiation_time = instantiation_start.elapsed().as_nanos();

    // Pass the input to the WASM module
    let args_start = std::time::Instant::now();
    let Ok(set_input) = instance.get_typed_func::<u32, u32>(&mut store, "set_input") else { anyhow::bail!("Failed to get set_input");};
    // Allocate wASM memory for the input
    let input_ptr = set_input.call(&mut store, input.len() as u32)? as usize;
    // Get the memory from the WASM module
    let Some(memory) = instance.get_memory(&mut store, "memory") else { anyhow::bail!("Failed to get WASM memory"); };
    // Write the input to the WASM memory
    let input_bytes = input.as_bytes();
    memory.data_mut(&mut store)[input_ptr..(input_ptr + input_bytes.len())].copy_from_slice(input_bytes);
    let args_time = args_start.elapsed().as_nanos();

    // Call the _start function
    let call_start = std::time::Instant::now();
    let func = instance.get_typed_func::<(), ()>(&mut store, "_start").unwrap();
    func.call(&mut store, ())?;
    let call_time = call_start.elapsed().as_nanos();

    // Get the result from the WASM module execution
    let result_start = std::time::Instant::now();
    let Ok(get_result_len) = instance.get_typed_func::<(), u32>(&mut store, "get_result_len") else { anyhow::bail!("Failed to get get_result_len");};
    let length = get_result_len.call(&mut store, ())? as usize;

    let Ok(get_result) = instance.get_typed_func::<(), u32>(&mut store, "get_result") else { anyhow::bail!("Failed to get get_result");};
    let content_ptr = get_result.call(&mut store, ())? as usize;

    let content = memory.data(&store)[content_ptr..(content_ptr + length)].to_vec();
    let result = String::from_utf8(content)?;
    let result_time = result_start.elapsed().as_nanos();

    let global_time = global_start.elapsed().as_nanos();

    //println!("From embedder:\n\tResult ptr: {}\n\tResut length: {}\n\tResult {}", content_ptr, length, result);

    println!("Timing (ns):\n\tSetup: {}\n\tLoad: {}\n\tInstantiation: {}\n\tArgs: {}\n\tCall: {}\n\tResult: {}\n\tGlobal: {}", 
            setup_time, load_time, instantiation_time, args_time, call_time, result_time, global_time);
    println!("Output: {}", result);

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