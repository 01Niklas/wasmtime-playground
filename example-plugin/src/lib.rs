use runtime::api::PluginParams;

#[no_mangle]
pub extern "C" fn start(pointer: i64) -> i32 {
    println!("Hello from Plugin, {:?}", pointer);
    let pointer: *const PluginParams = pointer as *const PluginParams;
    println!("Hello from Plugin, {:?}", pointer);
    let (lhs, rhs) = unsafe {
        ((*pointer).lhs, (*pointer).rhs)
    };
    let result = lhs + rhs;

    result
}
