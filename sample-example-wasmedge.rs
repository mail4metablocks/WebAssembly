// Import the WasmEdge library
extern crate wasmedge;

fn main() {
    // Load a WASM file into memory
    let wasm_bytes = std::fs::read("example.wasm").unwrap();

    // Create an instance of the WASM module
    let mut instance = wasmedge::Instance::new(&wasm_bytes).unwrap();

    // Call a function in the WASM module
    let result = instance.call("sum", &[1, 2, 3]).unwrap();

    // Print the result of the function call
    println!("The sum is: {}", result[0].unwrap());
}
