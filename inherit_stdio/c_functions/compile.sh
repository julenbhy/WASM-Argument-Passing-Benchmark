#!/bin/bash

WASI_SDK="/opt/wasi-sdk/bin/clang"  # Cambia esto por la ruta correcta a wasi-sdk
WASMTIME=wasmtime
# Obtiene el nombre del archivo .c como argumento
code_file=$1
basename=$(basename "$code_file" .c)

# Crea el directorio 'compiled' si no existe
mkdir -p compiled

# Lee el contenido del archivo original (que solo tiene la función `func`)
user_code=$(cat "$code_file")

# Agrega el código adicional antes de compilar
cat <<EOF > compiled/$basename.full.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

$user_code

const char *RESULT;
char __attribute__((export_name("get_result"))) *get_result() { return strdup(RESULT); }
size_t __attribute__((export_name("get_result_len"))) get_result_len() { return strlen(RESULT); }


int main(int argc, char *argv[]) { 
    RESULT = func(argv[1]); 
    printf("From WASM: \n\tResult ptr: %d\n\tResult len: %d\n\tResult content: %s\n", &RESULT, strlen(RESULT), RESULT);
    return 0; 
}
EOF

# Compila el código original usando el compilador regular
gcc compiled/$basename.full.c -o compiled/$basename

# Compila el código usando wasi-sdk
$WASI_SDK compiled/$basename.full.c -o compiled/$basename.wasm

# Precompila el código wasm usando wasmtime
$WASMTIME compile compiled/$basename.wasm -o compiled/$basename.cwasm
