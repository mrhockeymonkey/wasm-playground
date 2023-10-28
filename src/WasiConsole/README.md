# WasiConsole

## Local 

```powershell
dotnet build
wasmtime ./bin/Debug/net7.0/WasiConsole.wasm

# Hello from Wasm at 04/26/2023 20:55:04
```

# WasmEdge

Requires DockerDesktop 4.15+ (enable containerd feature: Settings > Features in development > Beta)

```powershell
function build-wasi-console {
    docker buildx build --platform wasi/wasm -t mrhockeymonkey/cs-wasi-console:latest . 
    docker push mrhockeymonkey/cs-wasi-console:latest
    docker container run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm mrhockeymonkey/cs-wasi-console:latest
}

```


## Spin

```powershell
docker build -t mrhockeymonkey/cs-wasi-console:latest . && docker push mrhockeymonkey/cs-wasi-console:latest

kubectl apply -f ..\..\k3d\cs-wasi-console.yaml  

docker buildx build --platform wasi/wasm -t cs-wasi-console .

docker container run --rm `
  --runtime=io.containerd.wasmedge.v1 `
  --platform=wasi/wasm \
  nigelpoulton/docker-wasm:0.1
```


docker container run --rm --name=dockerwasm `
  --runtime=io.containerd.wasmedge.v1 `
  --platform=wasi/wasm `
  nigelpoulton/docker-wasm:0.1