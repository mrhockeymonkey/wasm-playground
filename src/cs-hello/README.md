# cs-hello


```bash
# local build and run with wasmtime
dotnet build -c Release

wasmtime ./bin/Release/net7.0/cs-hello.wasm
# Hello from Wasm at 04/30/2023 10:20:27

# wasmedge doesnt work on windows only on linux?
wasmedge ./bin/Release/net7.0/cs-hello.wasm
# Hello from Wasm at 04/30/2023 10:20:54
```


# Docker Support
```bash
# wasmtime runtime is only included in a tech preview build NOT the most recent version
# use links from https://www.docker.com/blog/announcing-dockerwasm-technical-preview-2/

docker buildx build --platform wasi/wasm -t mrhockeymonkey/cs-hello:0.3 . 
docker push mrhockeymonkey/cs-hello:0.3

docker container run --rm --runtime=io.containerd.wasmtime.v1 --platform=wasi/wasm mrhockeymonkey/cs-hello:0.3

# docker: Error response from daemon: Others("error setting up module: could not load module from file: failed to read input file"): unknown.


docker container run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm mrhockeymonkey/cs-hello:0.3

# [2023-04-30 10:23:13.805] [error] loading failed: invalid path, Code: 0x20
# [2023-04-30 10:23:13.805] [error]     File name: "/var/run/desktop-containerd/daemon/io.containerd.runtime.v2.task/moby/516e6d598e33b99281bc55f3a743acecb47a24eda96155e607e5e384c2a6de30/rootfs/cs-hello.wasm"
# docker: Error response from daemon: Others("Invalid file path"): unknown.
```

# ContainerD Support 

```bash
# run using containerd-wasmtime-shim
sudo ctr image pull docker.io/mrhockeymonkey/cs-hello:0.3
sudo ctr run --rm --runtime=io.containerd.wasmtime.v1 docker.io/mrhockeymonkey/cs-hello:0.3 cs-hello
# <No Output but is running with out any errors...>
```