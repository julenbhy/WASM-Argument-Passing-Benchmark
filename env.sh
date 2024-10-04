#!/bin/bash

# Export paths
export CC="gcc"
export WASMTIME="/opt/wasmtime-v21.0.1-x86_64-linux/wasmtime"
export WASI_SDK="/opt/wasi-sdk-21"

# Optional: echo to confirm the paths have been set
echo "CC set to $CC"
echo "WASMTIME set to $WASMTIME"
echo "WASI_SDK set to $WASI_SDK"
