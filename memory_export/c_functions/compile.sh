#!/bin/bash

WASI_SDK="/opt/wasi-sdk/bin/clang"  # Replace with the actual path to wasi-sdk
WASMTIME=wasmtime
# Get the name of the C code file as an argument
code_file=$1
basename=$(basename "$code_file" .c)

# Create the 'compiled' directory if it doesn't exist
mkdir -p compiled

# Compile the code using the regular compiler
gcc $code_file -o compiled/$basename

# Compile the code using wasi-sdk
$WASI_SDK $code_file -o compiled/$basename.wasm

# Precompile the wasm code using wasmtime
$WASMTIME compile compiled/$basename.wasm -o compiled/$basename.cwasm

