# cs-hello


```bash
# local build and run with wasmtime
dotnet build -c Release

wasmtime ./bin/Release/net7.0/cs-hello.wasm
# Hello from Wasm at 04/30/2023 10:20:27
wasmedge ./bin/Release/net7.0/cs-hello.wasm
# Hello from Wasm at 04/30/2023 10:20:54
```


```powershell
# building a wasm docker image only works on windows for some reason
docker buildx build --platform wasi/wasm -t mrhockeymonkey/cs-hello:0.1 . 
docker push mrhockeymonkey/cs-hello:0.1
```

```bash
# but containerd is best used from a full ubuntu OS so switch back to linux for this

# dockers built in wasmedge seems to hang when running dotnet in windows and error on linux...
docker container run --rm \
  --runtime=io.containerd.wasmedge.v1 \
  --platform=wasi/wasm \
  mrhockeymonkey/cs-hello:0.2

# [2023-04-30 10:23:13.805] [error] loading failed: invalid path, Code: 0x20
# [2023-04-30 10:23:13.805] [error]     File name: "/var/run/desktop-containerd/daemon/io.containerd.runtime.v2.task/moby/516e6d598e33b99281bc55f3a743acecb47a24eda96155e607e5e384c2a6de30/rootfs/cs-hello.wasm"
# docker: Error response from daemon: Others("Invalid file path"): unknown.

# run using containerd-wasmtime-shim
sudo ctr image pull docker.io/mrhockeymonkey/cs-hello:0.2
sudo ctr run --rm --runtime=io.containerd.wasmtime.v1 docker.io/mrhockeymonkey/cs-hello:0.2 cs-hello
# ctr: Others("error setting up module: could not load module from file: failed to read input file"): unknown
```