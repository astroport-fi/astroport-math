build:
	wasm-pack build --target nodejs --scope astroport --release

run-tests: 
	cd test && npm i && npm run test