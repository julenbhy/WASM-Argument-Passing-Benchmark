use wasmtime::*;
use wasi_common::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    
    let args: Vec<String> = std::env::args().collect();

    //if args.len() < 2 {
    //    eprintln!("Usage: {} <wasm file>", args[0]);
    //    std::process::exit(1);
    //}

    // Version 1: Take multiple arguments and create a JSON object
    //let input = vec![" ".to_string(), parse_args(args.clone())];
    // Version 2: Take a single argument (already a JSON object)
    let input = vec![" ".to_string(), args[2].clone()];
    println!("Input: {:?}", input);

    // Time the entire process
    let global_start = std::time::Instant::now();

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().args(&input)?.build();
    let mut store = Store::new(&engine, wasi_ctx);

    // Load the module from disk
    let bytes = std::fs::read(&args[1]).unwrap();
    let start = std::time::Instant::now();
    let module = unsafe { Module::deserialize(&engine, bytes)? };
    println!("Deserialization time: {:?}ns", start.elapsed().as_nanos());

    //Instantiate the module
    let start = std::time::Instant::now();
    let instance_pre = linker.instantiate_pre(&module)?;
    println!("Preinstantiation time: {:?}ns", start.elapsed().as_nanos());

    let start = std::time::Instant::now();
    let instance = instance_pre.instantiate(&mut store).unwrap();
    println!("Instantiation time: {:?}ns", start.elapsed().as_nanos());


    // Call the _start function
    let start = std::time::Instant::now();
    let func = instance.get_typed_func::<(), ()>(&mut store, "_start").unwrap();
    func.call(&mut store, ())?;
    println!("Execution time: {:?}ns", start.elapsed().as_nanos());


    // Get the result from the WASM module execution
    let start = std::time::Instant::now();

    let Some(memory) = instance.get_memory(&mut store, "memory") else {
        anyhow::bail!("Failed to get WASM memory");
    };

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

    println!("Result retrieve time:: {:?}ns", start.elapsed().as_nanos());

    println!("Total time taken: {:?}ns", global_start.elapsed().as_nanos());

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