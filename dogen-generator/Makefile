build-wasm:
	wasm-pack build --target web --release
	mv pkg/* ../dogen-frontend/src/lib/engine/
	rm -r pkg