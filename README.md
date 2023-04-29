# wasm.net

My playground for everthing .net and wasm related...


## Technologies

Spin. Seems to act as a http gateway, serializes responses to wasm modules and returns responses

https://developer.fermyon.com/spin/index
https://github.com/deislabs/containerd-wasm-shims/blob/main/containerd-shim-spin-v1/quickstart.md alpha kubernetes support

Kubernetes has a concept of runtimeClasses https://github.com/kubernetes/enhancements/tree/master/keps/sig-node/585-runtime-class

https://github.com/containerd/runwasi


## How Blazor Works (I think...)

Blazor uses CoreCLR to produce IL.
That IL is passed to the dotnet runtime compiled into wasm by mono (since mono has llvm)
The IL is then interpreted (not JITed)
Blazor can also include native binaries which are compiled into WebAssembly by emscripten. 

## How Wasi.Sdk works (I think...)

`SteveSandersonMS/Wasi.Sdk` (Nuget) - https://github.com/SteveSandersonMS/dotnet-wasi-sdk
Allows compiling .net code to WASI compliant bytecode

This depends on `WebAssembly/wasi-sdk` which is a C/C++ compiler based on LLVM.

This also depends on the dotnet runtime/src/mono/wasm support which is built 

The regular dotnet build produces dlls which the Wasi.Sdk then compiles into a wasm along with a precompiled mono runtime. 
This can then be run using wasitime (a rust implementation of the wasi vm)

For asp.net there is more functionality:

? wgar is? UseWasiConnectionListener



---


# Idea 1
An asp.net application which manages wasm files
A set of worker nodes which host them with the desired config
... how would you reach them?
... if you went as far as removing even docker you would need to reinvent k8s ingress and other things


## Idea 2 (leading)
Portal for uploading/configuring wasm app which automates and applies all kubernetes config
to spin up a wasm runtime pod? basically an abstraction over helm/argocd?
Traditional docker, familliar, logging and ingress all work as before.

basically an argocd but for wasm
wasmcd

to poc:

- create a couple wasm binaries from a couple languages
- helm chart that can be used to run each
- come up with a config as code schema
- app to manage it. 


