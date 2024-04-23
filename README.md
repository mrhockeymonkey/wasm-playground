# wasm.net

My playground for everthing wasm/wasi related...

## Summary

`WebAssembly` is a portable bytecode format originally aimed to bring assembly-like programming to the web. It will run on the clients' browser and so needed to be hardware independant. 
Outside of a browser it can also run on a VM and runtime environments like wasmtime. It is langauge agnostic and aims to support any language.

The `Component Model` defines how WebAssembly modules can be composed together. WebAssembly modules may import functions, global variables, etc. from the host runtime, as well as export such items to the host. However, there is no standard way to combine modules at runtime, nor is there a standard way to marshal high-level types (e.g. strings and records) across module boundaries. The component model addresses this by introducing interfaces (WIT), an ABI for high level types liek strings and records, and a mechanism for dynamically linking modules. 

`WASI` is the WebAssembly Systems Interfaces and brings WASM to the server. On the Web, it uses the existing Web APIs provided by browsers. However outside of browsers, there's currently no standard set of APIs that WebAssembly programs can be written to for all platforms; windows, linux, android, etc. This makes it difficult to create truly portable non-Web WebAssembly programs. WASI is an initiative to fill this gap, with a clean set of APIs which can be implemented on multiple platforms by multiple engines

> Solomon Hykes, a co-founder of Docker, wrote in 2019, "If WASM+WASI existed in 2008, we wouldn't have needed to create Docker.

https://github.com/WebAssembly/WASI


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


