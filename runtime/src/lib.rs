use crate::api::PluginParams;

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "runtime")]
pub fn start(module_file: &str) -> anyhow::Result<()> {
    println!("Start Runtime");

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = wasmtime_wasi::WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = wasmtime::Store::new(&engine, wasi);

    let module = wasmtime::Module::from_file(&engine, module_file)?;

    let instance = linker.instantiate(&mut store, &module)?;

    let start_fn = linker
        .get(&mut store, "", "start")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<i64, i32>(&store)?;

    let memory = instance.get_memory(&mut store, "memory").ok_or(anyhow::anyhow!("Memory not found"))?;

    memory.write(&mut store, 0, &[0,0,0,42,0,0,0,73]).unwrap();

    let params = Box::new(PluginParams {
        lhs: 42,
        rhs: 73
    });

    let params_ptr: *const PluginParams = &* params;
    let params_ptr: i64 = params_ptr as i64;

    let result = start_fn.call(&mut store, params_ptr)?;

    println!("{result}");

    Ok(())
}
