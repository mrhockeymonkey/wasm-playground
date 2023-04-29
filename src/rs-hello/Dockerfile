FROM scratch
COPY ./target/wasm32-wasi/release/rs-hello.wasm /rs-hello.wasm
ENTRYPOINT [ "rs-hello.wasm" ]