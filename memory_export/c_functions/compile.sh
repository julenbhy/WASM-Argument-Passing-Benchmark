#!/bin/bash

# Define default paths if not set in the environment
WASI_SDK=${WASI_SDK_PATH:-"/opt/wasi-sdk"}
WASMTIME=${WASMTIME_PATH:-"wasmtime"}

# Gets the name of the .c file as an argument
code_file=$1
basename=$(basename "$code_file" .c)

# Creates the 'compiled' directory if it doesn't exist
mkdir -p compiled

# Reads the content of the original file (which only has the `func` function)
user_code=$(cat "$code_file")

# Adds additional code before compiling
cat <<EOF > compiled/$basename.full.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

$user_code

const char *INPUT;
char* __attribute__((export_name("set_input"))) *set_input(size_t size) {
    INPUT = (char*) malloc(size);
    return INPUT;
}

const char *RESULT;
char __attribute__((export_name("get_result"))) *get_result() { return strdup(RESULT); }
size_t __attribute__((export_name("get_result_len"))) get_result_len() { return strlen(RESULT); }

int main(int argc, char *argv[]) { 
    RESULT = func(INPUT); 
    printf("From WASM: \n\tResult ptr: %d\n\tResult len: %d\n\tResult content: %s\n", RESULT, strlen(RESULT), RESULT);
    return 0; 
}
EOF

# Compiles the original code using the regular compiler
gcc compiled/$basename.full.c -o compiled/$basename

# Compiles the code using wasi-sdk
$WASI_SDK/bin/clang --sysroot=$WASI_SDK/share/wasi-sysroot compiled/$basename.full.c -o compiled/$basename.wasm

# Precompiles the wasm code using wasmtime
$WASMTIME compile compiled/$basename.wasm -o compiled/$basename.cwasm
