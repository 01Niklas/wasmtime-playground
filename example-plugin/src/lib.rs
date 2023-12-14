use runtime::api::PluginParams;

#[no_mangle]
pub extern "C" fn start(pointer: i64) -> i32 {
    let pointer: *const PluginParams = pointer as *const PluginParams;

    let (lhs, rhs) = unsafe {
        ((*pointer).lhs, (*pointer).rhs)
    };
    let result = lhs + rhs;


    //TODO: Read memory

    result
}
