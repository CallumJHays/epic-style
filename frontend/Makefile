SHELL := /bin/bash

all:
	cargo web build --runtime library-es6 --target=wasm32-unknown-unknown --release
	mkdir -p ../static
	find target/wasm32-unknown-unknown/release -type f -name "*.wasm" | xargs -I {} cp {} ../static/frontend.wasm
	find target/wasm32-unknown-unknown/release -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} ../static/mounter.mjs