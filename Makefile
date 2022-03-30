
.PHONY: build
build:
	cargo build --target wasm32-wasi --release --manifest-path=rust-hello/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path=rust-static-assets/Cargo.toml
	cargo build --target wasm32-wasi --release --manifest-path=rust-outbound-http/Cargo.toml

	cd go-hello && make
	cd go-static-assets && make
	cd go-outbound-http && make
	cd assemblyscript-outbound-http && npm install && npm run asbuild

.PHONY: serve
serve:
	RUST_LOG=spin=trace spin up
