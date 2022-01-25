.PHONY: test

test:
	.vessel/.bin/0.6.2/moc $(shell vessel sources) -wasi-system-api -o Test.wasm test/*Test.mo && wasmtime Test.wasm
	rm -f Test.wasm
