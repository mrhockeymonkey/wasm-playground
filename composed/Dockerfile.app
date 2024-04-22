FROM scratch
COPY ./wasi-demo-app.wasm /wasi-demo-app.wasm
ENTRYPOINT [ "/wasi-demo-app.wasm" ]