#!/bin/bash

# Define default paths if not set in the environment
WASI_SDK=${WASI_SDK_PATH:-"/opt/wasi-sdk"}
WASMTIME=${WASMTIME_PATH:-"wasmtime"}

# Function to add dependencies to Cargo.toml
add_dependencies() {
  local crate_names=$(grep -Eo 'use [a-zA-Z0-9_]+::' "$1" | awk '{print $2}' | sed 's/::$//' | sort | uniq)
  for crate in $crate_names; do
    if ! grep -q "^$crate =" Cargo.toml; then
      echo "Adding dependency $crate to Cargo.toml"
      cargo add "$crate"
    fi
  done
}


# Check if a .rs file was passed as an argument
if [ $# -ne 1 ]; then
  echo "Usage: $0 file.rs"
  exit 1
fi

INPUT_FILE=$1

# Check if the file has a .rs extension
if [[ $INPUT_FILE != *.rs ]]; then
  echo "The file must have a .rs extension"
  exit 1
fi

# Check if the file exists
if [ ! -f $INPUT_FILE ]; then
  echo "The file $INPUT_FILE does not exist"
  exit 1
fi


# Define paths
TEMPLATE_FILE="builder/src/main_template.rs"
LIB_FILE="builder/src/main.rs"
CARGO_TOML_TEMPLATE_FILE="builder/Cargo_template.toml"
CARGO_TOML_FILE="builder/Cargo.toml"
TARGET_DIR="builder/target/wasm32-wasi/release"
OUTPUT_FILE=$(basename "$INPUT_FILE" .rs)

# Check if the template file exists
if [ ! -f $TEMPLATE_FILE ]; then
  echo "The file $TEMPLATE_FILE does not exist"
  exit 1
fi

# Copy the content of lib_template.rs to lib.rs
cp $TEMPLATE_FILE $LIB_FILE

# Copy the content of Cargo_template.toml to Cargo.toml
cp $CARGO_TOML_TEMPLATE_FILE $CARGO_TOML_FILE

# Concatenate the content of the input file to lib.rs
cat $INPUT_FILE >> $LIB_FILE

# Add dependencies to Cargo.toml if any are found
cd builder
add_dependencies "../$INPUT_FILE"

# Compile the Rust project with cargo-component
cargo build --release --target wasm32-wasi

# Check if the compilation was successful
if [ $? -ne 0 ]; then
  echo "Compilation failed"
  exit 1
fi

# Copy the resulting binary to the current directory and name it after the input file
cd ..

mkdir -p compiled
cp $TARGET_DIR/builder.wasm "compiled/$OUTPUT_FILE.wasm"

echo "Compilation successful. Binary saved as compiled/$OUTPUT_FILE"

# Precompole the wasm file using wasmtime
wasmtime compile "compiled/$OUTPUT_FILE.wasm" -o "compiled/$OUTPUT_FILE.cwasm"
