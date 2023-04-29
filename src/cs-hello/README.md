# cs-hello


```bash
# local build and run with wasmtime
dotnet build -c Release

wasmtime ./bin/Release/net7.0/cs-hello.wasm
```


```powershell
# building a wasm docker image only works on windows for some reason
docker buildx build --platform wasi/wasm -t mrhockeymonkey/cs-hello:0.1 . 
docker push mrhockeymonkey/cs-hello:0.1
```

```bash
# but containerd is best used from a full ubuntu OS so switch back to linux for this

# dockers buit in wasmedge seems to hang when running dotnet...

# run using containerd-wasmtime-shim
sudo ctr image pull docker.io/mrhockeymonkey/cs-hello:0.2
sudo ctr run --rm --runtime=io.containerd.wasmtime.v1 docker.io/mrhockeymonkey/cs-hello:0.2 cs-hello
# ctr: Others("error setting up module: could not load module from file: failed to read input file"): unknown
```