# Rust Hello World App

Requires:

- Rust
- Wasmtime
- DockerDesktop + containerd support
- containerd

```bash
rustup target add wasm32-wasi

cargo build --target wasm32-wasi --release

wasmtime target/wasm32-wasi/release/rs-hello.wasm
# Hello, world!

# Now compile the wasmtime shim from containerd/runwasi repo
cd ~/code/runwasi
cargo build --release
install ~/code/runwasi/target/release/containerd-shim-wasmtime-v1 /usr/local/bin
```

```powershell
# for some reason this only works on windows
# build wasi container
docker buildx build --platform wasi/wasm -t mrhockeymonkey/rs-hello:0.1 .
docker push mrhockeymonkey/rs-hello:0.1
```

```bash
# but containerd is best used from a full ubuntu OS so switch back to linux for this

# run using dockers buit in wasmedge
docker container run --rm --name=dockerwasm \
  --runtime=io.containerd.wasmedge.v1 \
  --platform=wasi/wasm \
  mrhockeymonkey/rs-hello:0.1

# run using containerd-wasmtime-shim
sudo ctr image pull docker.io/mrhockeymonkey/rs-hello:0.1
sudo ctr run --rm --runtime=io.containerd.wasmtime.v1 docker.io/mrhockeymonkey/rs-hello:0.1 rs-hello
```