DIST=./target/wasm32-wasi/release

# cleanup
rm $DIST/db-dep.wasm
rm $DIST/books-api-dep.wasm
rm $DIST/cmd.wasm
rm $DIST/tmp.wasm
rm composed.wasm

cargo component build -p db-dep --release
mv $DIST/db_dep.wasm $DIST/db-dep.wasm

cargo component build -p books-api-dep --release
mv $DIST/books_api_dep.wasm $DIST/books-api-dep.wasm

cargo component build -p cmd --release

wasm-tools compose $DIST/books-api-dep.wasm -d $DIST/db-dep.wasm -o $DIST/tmp.wasm
wasm-tools compose $DIST/cmd.wasm -d $DIST/tmp.wasm -o composed.wasm