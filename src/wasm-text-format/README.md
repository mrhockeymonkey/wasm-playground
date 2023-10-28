# Wasm Text Format

WebAssembly can be written in a low level text format described here

https://webassembly.github.io/spec/core/text/conventions.html

This can then be compiled to the binary format in a few ways.

This is some experimentation with "pure" web assembly to get an idea of how it works lower level

We can compile the text format in `.wasm` files using WebAssembly binary tools.

https://github.com/WebAssembly/wabt

```bash
~/Downloads/wabt-1.0.34-ubuntu/wabt-1.0.34/bin/wat2wasm add.wat

xxd add.wasm
00000000: 0061 736d 0100 0000 010a 0260 027f 7f01  .asm.......`....
00000010: 7f60 0000 0303 0200 0107 1201 0e63 616c  .`...........cal
00000020: 6c65 7246 756e 6374 696f 6e00 010a 1302  lerFunction.....
00000030: 0700 2000 2001 6a0b 0900 4105 4107 1000  .. . .j...A.A...
00000040: 1a0b                                     ..
```

So that produces a valid wasm file we can run with an embedder like wasmtime

```bash
wasmtime run add.wasm --invoke callerFunction
echo $?
0
```

In theory those numbers got added but if I wanted to write that out to the console I would need to provide the wasm file with a method that would take care of that on this host. As far as I understand this would be the job of the embedder.

Wasmtime has an example of how to do this which I have adapted

```bash
cd add-display/
cargo build
cargo run


Compiling module...
Initializing...
Creating callback...
Instantiating module...
Extracting export...
Calling export...
Calling back...
The result was 12 # <--- there is our answer>
Done.
```


### Wasmtime

I tried compiling a `.wat` file using wasmtime but I'm not really sure what this even does... 
There seems to be a thing called "compiled wasm"

```pwsh
wasmtime compile -o add-wasmtime.cwasm add.wat 

xxd add-wasmtime.cwasm | more
00000000: 7f45 4c46 0201 01c8 0000 0000 0000 0000  .ELF............
```

So its not a wasm file? Ill just leave that one alone for now

