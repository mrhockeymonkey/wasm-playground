# RS-Hello

Requires:

- Rust
- Wasmtime

```bash
rustup target add wasm32-wasi

cargo build --target wasm32-wasi --release

wasmtime target/wasm32-wasi/release/rs-hello.wasm
# Hello, world!

# Now compile the wasmtime shim from containerd/runwasi repo
cd ~/code/runwasi
cargo build --release
install ~/code/runwasi/target/release/containerd-shim-wasmtime-v1 /usr/local/bin


#---------------- not working yet
# build wasi container
docker buildx build --platform wasi/wasm -t rs-hello:0.1 .

docker container run --rm --name=dockerwasm \
  --runtime=io.containerd.wasmedge.v1 \
  --platform=wasi/wasm \
  rs-hello:0.1

sudo ctr run --rm --runtime=io.containerd.wasmedge.v1 docker.io/library/wasmtest:latest testwasm
```