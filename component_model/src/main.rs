use wasmtime::{Engine, Result, Store};
use wasmtime::component::{Linker, Component};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder, ResourceTable};

fn main() -> Result<()> {

    let args: Vec<String> = std::env::args().collect();
    // Version 1: Take multiple arguments and create a JSON object
    //let input = parse_args(args.clone());
    // Version 2: Take a single argument (already a JSON object)
    let input = args[2].clone();
    println!("Input: {:?}", input);

    let mut result = [wasmtime::component::Val::String("".into())];
        
    let engine = Engine::default();
    let mut linker = Linker::<MyState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    //wasmtime_wasi::add_to_linker_async(&mut linker)?; // Should be sync or async? https://docs.rs/wasmtime/21.0.1/wasmtime/struct.Config.html#asynchronous-wasm

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, MyState { ctx: wasi_ctx, table: ResourceTable::new(),},);

    // Load the component from disk
    let bytes = std::fs::read(&args[1]).unwrap();
    let component = unsafe { Component::deserialize(&engine, bytes)? };

    // Instantiate the component
    let instance_pre = linker.instantiate_pre(&component)?;
    let instance = instance_pre.instantiate(&mut store).unwrap();

    // Call the `func-wrapper` function
    let func = instance.get_func(&mut store, "func-wrapper").expect("func-wrapper export not found");
    func.call(&mut store, &[wasmtime::component::Val::String(input.into())], &mut result)?;

    println!("From rust:\n\tResult: {:?}", result);

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