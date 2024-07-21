# Composed

An example of composing multiple components together.

```bash
cargo install wasm-tools

# compose into a single wasm (requires renaming to kebab case)
source build.sh

  
wasm-tools component wit composed.wasm # to see resulting wit
wasmtime run composed.wasm

```

## WIT

See: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
See: https://component-model.bytecodealliance.org/creating-and-consuming.html

Each component defines a world and within that world can be many types and instances

The below exports a function which means it is available to call from **outside**, i.e. via wasmtime
```plain
package example:component;

world example {
    export add: func(x: s32, y: s32) -> s32;
}
```

However the above will not let you call the function from another component. To do that you must export and interface

```plain
package docs:adder@0.1.0;

interface add {
    add: func(a: u32, b: u32) -> u32;
}

world adder {
    export add;
}
```
