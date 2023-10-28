use wasmtime::*;

struct MyState {
    name: String,
    count: usize,
}

fn main() {
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./src/add-display.wat").unwrap();

    println!("Initializing...");
    let mut store = Store::new(
        &engine,
        MyState { // this isnt used in this example
            name: "hello, world!".to_string(),
            count: 0,
        },
    );

    // Our wasm module we'll be instantiating requires one imported function.
    // the function takes no parameters and returns no results. We create a host
    // implementation of that function here, and the `caller` parameter here is
    // used to get access to our original `MyState` value.
    println!("Creating callback...");

    // This function will create a new Func which, when called, will execute the given Rust closure.
    let display_func = Func::wrap(&mut store, |result: i32| {
        println!("Calling back...");
        println!("The result was {result}");
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    let imports = [display_func.into()];
    let instance = Instance::new(&mut store, &module, &imports).unwrap();

    // Next we poke around a bit to extract the `run` function from the module.
    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "callerFunction").unwrap();

    // And last but not least we can call it!
    println!("Calling export...");
    run.call(&mut store, ()).unwrap();

    println!("Done.");
}
