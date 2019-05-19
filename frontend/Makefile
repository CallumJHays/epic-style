SHELL := /bin/bash
TARGET := wasm32-unknown-unknown

all:
	mkdir -p ../static
	touch ../static/frontend.wasm ../static/mounter.mjs
	# reset the symlinks
	
	cargo web build --runtime=library-es6 --target=${TARGET}
	find target/${TARGET}/debug -type f -name "*.wasm" | xargs -I {} ln -s {} ../static/frontend.wasm
	find target/${TARGET}/debug -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} ln -s {} ../static/mounter.mjs
	
	cargo web start --auto-reload --target=${TARGET}