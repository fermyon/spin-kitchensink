.PHONY: build
build:
	zig build-exe -O ReleaseSmall -target wasm32-wasi src/main.zig
	zig build-exe -O ReleaseSmall -target wasm32-wasi hello.c -lc