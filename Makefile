js:
	EMMAKEN_CFLAGS=" -s USE_SDL=2" cargo build --target asmjs-unknown-emscripten
	cp target/asmjs-unknown-emscripten/debug/pewpew.js docs/

serve: js
	cd docs/
	python -m SimpleHTTPServer 8000

clean:
	cargo clean
	rm -f docs/pewpew.js
