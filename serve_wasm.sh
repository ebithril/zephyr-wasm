#!/bin/bash
# 1. Copy the compiled wasm to the current directory
cp target/wasm32-unknown-unknown/release/zephyr-webasm.wasm .

# 2. Start a simple web server
echo "Starting server at http://localhost:8080"
python3 -m http.server 8080
