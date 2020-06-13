// use nodejs_sys::{
//     napi_callback_info, napi_create_double, napi_create_function, napi_env, napi_get_cb_info,
//     napi_get_value_double, napi_set_named_property, napi_value,
// };

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_get_cb_info, napi_get_undefined,
    napi_get_buffer_info, napi_set_named_property, napi_value, napi_create_double, napi_get_typedarray_info,
    napi_status, napi_get_element, napi_get_value_double, napi_get_array_length
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

pub unsafe extern "C" fn print_array(env: napi_env, info: napi_callback_info) -> napi_value {
    // creating a buffer of arguments
    let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;
    // getting arguments
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );

    // this is the array the was passed in
    let napi_val: napi_value = buffer[0];

    // get length
    let mut length: u32 = 0;
    let get_length_success: napi_status = napi_get_array_length(
        env,
        napi_val,
        &mut length as *mut u32
    );

    for i in 0..length {
        // create pointer to get value
        let mut result: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();

        let success: napi_status = napi_get_element(
            env,
            napi_val,
            i,
            result.as_mut_ptr()
        );

        // convert to a double
        let mut val = 0 as f64;
        napi_get_value_double(env, result[0], &mut val);

        println!("arr[{}] {:?}", i, val);
    }

    // sum the array contents
    let mut sum: u32 = 0;

    // creating the return value
    let mut local: napi_value = std::mem::zeroed();
    napi_create_double(env, sum as f64, &mut local);

    // returning the result
    local
}

pub unsafe extern "C" fn sum_u_32_array(env: napi_env, info: napi_callback_info) -> napi_value {
    // creating a buffer of arguments
    let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;
    // getting arguments
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    let mut length: usize = 0;
    let mut arr_type: napi_typedarray_type = std::mem::zeroed();

    // creating a buffer where data can be placed
    let mut ve: Vec<u32> = Vec::new();
    let mut raw = ve.as_mut_ptr();
    let mut_ref: &mut *mut u32 = &mut raw;
    let raw_ptr: *mut *mut _ = mut_ref as *mut *mut _;

    let napi_val = buffer[0];

    // getting length by passing null buffer
    napi_get_typedarray_info(
        env,
        napi_val,
        &mut arr_type,
        &mut length,
        raw_ptr as *mut *mut c_void,
        std::ptr::null_mut(),
        std::ptr::null_mut()
    );

    // assert_eq!(arr_type, napi_uint32_array);

    let s: Vec<u32> = Vec::from_raw_parts(raw, length, length);

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

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    let p1 = CString::new("sumBuffer").expect("CString::new failed");
    let p2 = CString::new("printArray").expect("CString::new failed");

    let mut local1: napi_value = std::mem::zeroed();
    let mut local2: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        p1.as_ptr(),
        5,
        Some(sum_u_32_array),
        std::ptr::null_mut(),
        &mut local1,
    );
    napi_create_function(
        env,
        p2.as_ptr(),
        5,
        Some(print_array),
        std::ptr::null_mut(),
        &mut local2,
    );
    napi_set_named_property(env, exports, p1.as_ptr(), local1);
    napi_set_named_property(env, exports, p2.as_ptr(), local2);
    exports
}
