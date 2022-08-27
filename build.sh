#!/bin/sh

# Make sure you have rust, wasm-pack and sfz (cargo install sfz) installed

wasm-pack build --release --target web --features wasm_thread/es_modules

sfz -r --coi
