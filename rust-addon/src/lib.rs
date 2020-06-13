// use nodejs_sys::{
//     napi_callback_info, napi_create_double, napi_create_function, napi_env, napi_get_cb_info,
//     napi_get_value_double, napi_set_named_property, napi_value,
// };

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_get_cb_info, napi_get_undefined,
    napi_get_buffer_info, napi_set_named_property, napi_value, napi_create_double, napi_get_typedarray_info
};
use nodejs_sys::napi_typedarray_type;
use nodejs_sys::napi_typedarray_type::napi_uint32_array;
use std::ffi::CString;
use std::ffi::c_void;

// pub unsafe extern "C" fn add(env: napi_env, info: napi_callback_info) -> napi_value {
// // creating a buffer where napi_value of argument be written
//     let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
// // max number of arguments
//     let mut argc = 2 as usize;
// // getting arguments and value of this
//     napi_get_cb_info(
//         env,
//         info,
//         &mut argc,
//         buffer.as_mut_ptr(),
//         std::ptr::null_mut(),
//         std::ptr::null_mut(),
//     );
// // converting napi to f64
//     let mut x = 0 as f64;
//     let mut y = 0 as f64;
//     napi_get_value_double(env, buffer[0], &mut x);
//     napi_get_value_double(env, buffer[1], &mut y);
// // creating the return value
//     let mut local: napi_value = std::mem::zeroed();
//     napi_create_double(env, x + y, &mut local);
// // returning the result
//     local
// }

pub unsafe extern "C" fn sumBuffer(env: napi_env, info: napi_callback_info) -> napi_value {
    let mut null_mut = std::ptr::null_mut();

    // creating a buffer of arguments
    let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;
    // getting arguments
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        null_mut,
        null_mut,
    );
    let mut length: usize = 0;
    let mut offset: usize = 0;
    let mut arr_type: napi_typedarray_type = std::mem::zeroed();

    // getting length by passing null buffer
    napi_get_typedarray_info(env, buffer[0], &mut arr_type, &mut length, null_mut, null_mut, &mut offset);
    let size = length - offset;

    assert_eq!(arr_type, napi_uint32_array);

    // creating a buffer where data can be placed
    let mut ve: Vec<u32> = Vec::with_capacity(size);
    let mut raw = ve.as_mut_ptr();
    let mut_ref: &mut *mut u32 = &mut raw as *mut *mut _;
    let raw_ptr: *mut *mut u32 = mut_ref as *mut *mut _;

    // getting the raw data from napi_value
    let _s = napi_get_typedarray_info(
        env,
        buffer[0],
        null_mut,
        null_mut,
        raw_ptr as *mut *mut c_void,
        null_mut,
        null_mut
    );
    // let _s = napi_get_typedarray_info(env, buffer[0], std::ptr::null_mut(), std::ptr::null_mut(), void_cast, std::ptr::null_mut(), std::ptr::null_mut());
    let s: Vec<u32> = Vec::from_raw_parts(raw, size, size);

    // println!("hello {:?}", s);

    // sum the array contents
    let mut sum: u32 = 0;

    for val in &s {
        sum += *val;
    }

    // telling rust not manage the vectors
    std::mem::forget(ve);
    std::mem::forget(s);

    // creating the return value
    let mut local: napi_value = std::mem::zeroed();
    napi_create_double(env, sum as f64, &mut local);

    // returning the result
    local
}

// pub unsafe extern "C" fn sumBuffer(env: napi_env, info: napi_callback_info) -> napi_value {
//     // creating a buffer of arguments
//     let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
//     let mut argc = 1 as usize;
//     // getting arguments
//     napi_get_cb_info(
//         env,
//         info,
//         &mut argc,
//         buffer.as_mut_ptr(),
//         std::ptr::null_mut(),
//         std::ptr::null_mut(),
//     );
//     let mut len = 0;
//     // let mut type: napi_typedarray_type = 0;
//
//     // getting length by passing null buffer
//     napi_get_buffer_info(env, buffer[0], std::ptr::null_mut(), &mut len);
//     let size = len as usize;
//
//     // creating a buffer where data can be placed
//     let mut ve: Vec<u8> = Vec::with_capacity(size);
//     let mut raw = ve.as_mut_ptr();
//     let mut_ref: &mut *mut u8 = &mut raw;
//     let raw_ptr: *mut *mut u8 = mut_ref as *mut *mut _;
//     let void_cast: *mut *mut c_void = raw_ptr as *mut *mut c_void;
//
//     // getting the raw data from napi_value
//     let mut cap = 0;
//     let _s = napi_get_buffer_info(env, buffer[0], void_cast, &mut cap);
//     let s: Vec<u8> = Vec::from_raw_parts(raw, cap as usize, size);
//
//     // println!("hello {:?}", s);
//
//     // sum the array contents
//     let mut sum: f64 = 0.0;
//
//     for val in &s {
//         sum += *val as f64;
//     }
//
//     // telling rust not manage the vectors
//     std::mem::forget(ve);
//     std::mem::forget(s);
//
//     // creating the return value
//     let mut local: napi_value = std::mem::zeroed();
//     napi_create_double(env, sum, &mut local);
//
//     // returning the result
//     local
// }

// #[no_mangle]
// pub unsafe extern "C" fn napi_register_module_v1(
//     env: napi_env,
//     exports: napi_value,
// ) -> nodejs_sys::napi_value {
// // creating a function name
//     let p = CString::new("add").expect("CString::new failed");
//     let mut local: napi_value = std::mem::zeroed();
// // creating the function
//     napi_create_function(
//         env,
//         p.as_ptr(),
//         5,
//         Some(add),
//         std::ptr::null_mut(),
//         &mut local,
//     );
// // setting function as property
//     napi_set_named_property(env, exports, p.as_ptr(), local);
// // returning exports
//     exports
// }


#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    let p = CString::new("sumBuffer").expect("CString::new failed");
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
        p.as_ptr(),
        5,
        Some(sumBuffer),
        std::ptr::null_mut(),
        &mut local,
    );
    napi_set_named_property(env, exports, p.as_ptr(), local);
    exports
}
