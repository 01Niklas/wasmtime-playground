use anyhow::Result;
use wasmtime::*;


fn main() -> Result<()> {
    let mut plugin = MyPlugin::new("examples/add/add.wat");
    let result = plugin.add(4, 3);
    println!("Result: {}", result);

    Ok(())
}


struct PluginState {

}

struct MyPlugin {
    store: Store<PluginState>,
    run_fn: TypedFunc<(i32, i32), i32>
}

impl MyPlugin {

    pub fn new(wat_path: &'static str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, wat_path)
            .unwrap();

        let mut store = Store::new(
            &engine,
            PluginState {
            },
        );

        let imports = [];
        let instance = Instance::new(&mut store, &module, &imports)
            .unwrap();

        let run = instance.get_typed_func::<(i32,i32), i32>(&mut store, "add")
            .unwrap();

        Self {
            store,
            run_fn: run,
        }
    }

    pub fn add(&mut self, lhs: i32, rhs: i32) -> i32 {
        self.run_fn.call(&mut self.store, (lhs, rhs)).unwrap()
    }
}
