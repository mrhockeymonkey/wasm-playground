# Wasm Text Format

WebAssembly can be written in a low level text format described here

https://webassembly.github.io/spec/core/text/conventions.html

This can then be compiled to the binary format in a few ways

### WebAssembly Binary Toolkit

https://github.com/WebAssembly/wabt

```pwsh
& "C:\Users\Scott\Downloads\wabt-1.0.34-windows.tar\wabt-1.0.34-windows\wabt-1.0.34\bin\wat2wasm.exe" -o gcd-wat2wasm.wasm gcd.wat 

# produces a 1KB file gcd-wat2wasm.wasm
```

### Wasmtime

This seems to ...

```pwsh
// using wasmtime
wasmtime compile -o gcd-wasmtime.cwasm gcd.wat 

```

