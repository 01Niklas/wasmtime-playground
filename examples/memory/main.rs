use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use wasmtime::*;

use pointer::Pointer;

mod pointer;


fn main() -> Result<()> {
    let mut plugin = MyPlugin::new("examples/memory/memory.wat")?;
    let pointer = Pointer(0x1000);
    plugin.write(pointer);

    let result = {
        let Pointer(pointer) = pointer;
        let pointer = usize::from(pointer);
        let result = Vec::from(&plugin.memory.data(&mut plugin.store)[pointer..]);
        let mut result = std::io::Cursor::new(result);
        result.read_u32::<LittleEndian>()? //WebAssembly uses LE: https://webassembly.org/docs/portability/
    };
    println!("Result: {}", result);

    Ok(())
}


struct PluginState {}

struct MyPlugin {
    store: Store<PluginState>,
    memory: Memory,
    run_fn: TypedFunc<pointer::Wasm, ()>
}

impl MyPlugin {

    pub fn new(wat_path: &'static str) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wat_path)?;

        let mut store = Store::new(
            &engine,
            PluginState {
            },
        );

        let imports = [];
        let instance = Instance::new(&mut store, &module, &imports)?;

        let memory = instance.get_memory(&mut store, "memory")
            .ok_or(anyhow!("failed to find 'memory' export."))?;

        let run = instance.get_typed_func::<pointer::Wasm, ()>(&mut store, "write")?;

        Ok(Self {
            store,
            memory,
            run_fn: run,
        })
    }

    pub fn write(&mut self, pointer: Pointer) {
        let wasm_pointer = pointer.into();
        self.run_fn.call(&mut self.store, wasm_pointer).unwrap()
    }
}
