.PHONY: build
build:
	tinygo build -target=wasi -gc=leaking -o main.wasm main.go

. PHONY: serve
serve:
	RUST_LOG=spin=trace spin up --file spin.toml
