use wasmtime::{Engine, Result, Store};
use wasmtime::component::{Linker, Component};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder, ResourceTable};

fn main() -> Result<()> {

    let args: Vec<String> = std::env::args().collect();
    // Version 1: Take multiple arguments and create a JSON object
    //let input = parse_args(args.clone());
    // Version 2: Take a single argument (already a JSON object)
    let input = args[2].clone();
    //println!("Input: {:?}", input);

    let mut result = [wasmtime::component::Val::String("".into())];
    
    // Time the entire process
    let global_start = std::time::Instant::now();

    // Set up the WASI environment
    let setup_start = std::time::Instant::now();
    let engine = Engine::default();
    let mut linker = Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    //wasmtime_wasi::add_to_linker_async(&mut linker)?; // Should be sync or async? https://docs.rs/wasmtime/21.0.1/wasmtime/struct.Config.html#asynchronous-wasm

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, MyState { ctx: wasi_ctx, table: ResourceTable::new(),},);
    let setup_time = setup_start.elapsed().as_nanos();

    // Load the component from disk
    let load_start = std::time::Instant::now();
    let bytes = std::fs::read(&args[1]).unwrap();
    let component = unsafe { Component::deserialize(&engine, bytes)? };
    let load_time = load_start.elapsed().as_nanos();

    // Instantiate the component
    let instantiation_start = std::time::Instant::now();
    let instance_pre = linker.instantiate_pre(&component)?;
    let instance = instance_pre.instantiate(&mut store).unwrap();
    let instantiation_time = instantiation_start.elapsed().as_nanos();

    // Call the `func-wrapper` function
    let call_start = std::time::Instant::now();
    let func = instance.get_func(&mut store, "func-wrapper").expect("func-wrapper export not found");
    func.call(&mut store, &[wasmtime::component::Val::String(input.into())], &mut result)?;
    let call_time = call_start.elapsed().as_nanos();

    // Format the result
    let result_start = std::time::Instant::now();
    let result = match &result[0] {
        wasmtime::component::Val::String(s) => s.clone(),
        _ => "".into(),
    };
    let result_time = result_start.elapsed().as_nanos();

    let global_time = global_start.elapsed().as_nanos();

    let args_time = 0; // Not any explicit time spent on args
    println!("Times (ns):\n\tSetup: {}\n\tLoad: {}\n\tInstantiation: {}\n\tArgs: {}\n\tCall: {}\n\tResult: {}\n\tGlobal: {}", 
            setup_time, load_time, instantiation_time, args_time, call_time, result_time, global_time);
    println!("Output: {}", result);

    Ok(())
}


struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
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