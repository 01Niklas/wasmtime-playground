# Plugins mechanism in Rust

## Idea
> Using Plugin mechanism for dealing with other scenarios or languages without putting all that stuff in production code

## Useful links
- Useful `rust` stuff: https://doc.rust-lang.org/rust-by-example/ or https://docs.rs
- Plugin mechanism by `Extism`:  https://github.com/extism/extism/tree/main/runtime
- Link to `Wasmtime` doku: https://docs.rs/wasmtime/latest/wasmtime/index.html
  - Memory in wasmtime: https://docs.wasmtime.dev/api/wasmtime/struct.Memory.html#method.data
- Link to the `WebAssembly Binary Toolkit`: https://github.com/WebAssembly/wabt
  - Transforming wasm to wat: https://manpages.ubuntu.com/manpages/focal/man1/wasm2wat.1.html
- Example for using `Web Assembly`: https://surma.dev/things/rust-to-webassembly/

## Problems: 
- How to deal with memory? 
  - how can we access parameters inside plugin?
