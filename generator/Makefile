build-wasm:
	wasm-pack build --target web --release
	mv pkg/* ../frontend/src/lib/engine/
	rm -r pkg