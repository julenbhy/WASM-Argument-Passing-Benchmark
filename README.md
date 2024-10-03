# WASM Argument Passing Benchmark

This repository contains a benchmark suite designed to evaluate and compare various methods of passing arguments to WebAssembly (WASM) modules. WebAssembly provides several techniques to interact with modules from an embedder, each with different trade-offs in terms of performance, memory efficiency, and complexity. 

The goal of this benchmark is to provide a detailed analysis of these argument-passing methods, allowing developers to make informed decisions about which technique is best suited for their use cases. Not only will the total execution time be measured, but specific stages such as module instantiation, function invocation, and memory management will also be timed. This granularity will help identify potential overhead introduced by each method.

## Argument Passing Methods

The following methods of argument passing will be benchmarked:

### 1. Inherit_stdio

In this method, the embedder passes arguments via the standard input/output (stdio) streams, which the WASM module inherits. The module reads from the stdin or writes to stdout during its execution. This method is typically used when the WASM module is designed to run as a standalone application, mimicking the behavior of traditional command-line programs. 

#### Pros:
- Development simplicity: The embedder does not need to interact with the WASM module's internal memory or functions.
- Suitable for command-line-like WASM applications.

#### Cons:
- Limited flexibility: The argument types are constrained by the text-based nature of stdin/stdout.
- Potentially less efficient for binary data or complex argument structures.

### 2. Export Memory

In this method, the WASM module exports a memory space that the embedder can interact with directly. The module also provides a function to allocate a buffer within this memory. The embedder writes input data into this buffer before invoking the module's main function, which then reads and processes the data from the pre-allocated memory space.

#### Pros:
- More efficient for passing binary or structured data.
- Allows the embedder to directly manipulate memory, reducing the need for intermediate data copying.

#### Cons:
- Increases complexity, as the embedder must handle memory allocation and pointer manipulation.
- Requires careful management of memory boundaries and potential alignment issues.

### 3. Component Model

The Component Model (introduced as part of WASI Preview 2 ) is a proposed extension to WebAssembly aimed at enabling better modularization and interoperability between WASM modules and host applications. In this model, richer type definitions can be described using WIT (Wasm Interface Types), which allows more complex data types like strings, arrays, and user-defined structures to be passed between the embedder and the module.

The way these types are mapped to low-level bytes is governed by the Canonical ABI (Application Binary Interface). A WASM component is essentially a core module wrapped with a WIT interface that specifies its imports and exports. 

Although the component model is designed to communicate different modules, we can take advantage of it to communicate the embedder with the module using complex data types.

Compiling the component model requires the installation of additional tools. More information can be found in the README within the component_model directory.

#### Pros:
- Strongly typed interface, making it easier to pass and return complex data types.
- Enables smooth two-way communication between the embedder and the module.
- Increased modularity, allowing for better composition of WASM libraries and modules.

#### Cons:
- Still in the proposal phase, so not yet widely supported.
- Could introduce additional overhead due to type translation between WIT and the core module's memory.
- Not compatible with wassi-threads


## Add New Functions

This benchmark suite has been developed as part of a project to port WASM to OpenWhisk. Therefore, with the intention of making it as user-friendly as possible, developers only need to implement a function that follows the signature:

- In C: `char* func(char* json)`
- In Rust: `pub fn func(json: serde_json::Value) -> result<serde_json::Value, anyhow::Error>`

For reference, you can find example functions in the files `add.c` and `add.rs`, included in each version of the embedder. To create new functions, simply copy one of these examples and modify the code.

Then, just compile it with:

```bash
./build.sh func_name.c
```


The embedders load WASM modules precompiled with Wasmtime (`.cwasm`). The precompilation process is done using Wasmtime CLI. The embedders are developed with Wasmtime 21.0.1, so they will only work if the same version of Wasmtime CLI is installed in the system.


## Execution

To execute the desired function (once compiled), navigate to the root directory of the chosen embedder and run the following command:

```bash
cargo run --release c_functions/compiled/func_name.cwasm arg1 arg2
```

## Software setup

- Wasmtime CLI 21.0.1
```sh

```

- WASI-SDK 21
```sh
curl -sL https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-21/wasi-sdk-21.0-linux.tar.gz | sudo tar -xz -C /opt/ && sudo mv /opt/wasi-sdk-21.0 /opt/wasi-sdk
```

- Cargo
```sh
curl https://sh.rustup.rs -sSf | sh
```

- cargo-component
```sh
cargo install cargo-component
```