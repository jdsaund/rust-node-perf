use nodejs_sys::{
    napi_callback_info, napi_create_double, napi_create_function, napi_env, napi_get_cb_info,
    napi_get_value_double, napi_set_named_property, napi_value,
};
use std::ffi::CString;

pub unsafe extern "C" fn add(env: napi_env, info: napi_callback_info) -> napi_value {
// creating a buffer where napi_value of argument be written
    let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
// max number of arguments
    let mut argc = 2 as usize;
// getting arguments and value of this
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
// converting napi to f64
    let mut x = 0 as f64;
    let mut y = 0 as f64;
    napi_get_value_double(env, buffer[0], &mut x);
    napi_get_value_double(env, buffer[1], &mut y);
// creating the return value
    let mut local: napi_value = std::mem::zeroed();
    napi_create_double(env, x + y, &mut local);
// returning the result
    local
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
// creating a function name
    let p = CString::new("myFunc").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();
// creating the function
    napi_create_function(
        env,
        p.as_ptr(),
        5,
        Some(add),
        std::ptr::null_mut(),
        &mut local,
    );
// setting function as property
    napi_set_named_property(env, exports, p.as_ptr(), local);
// returning exports
    exports
}
