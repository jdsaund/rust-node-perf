// use nodejs_sys::{
//     napi_callback_info, napi_create_double, napi_create_function, napi_env, napi_get_cb_info,
//     napi_get_value_double, napi_set_named_property, napi_value,
// };

use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_env, napi_get_cb_info, napi_get_undefined,
    napi_get_buffer_info, napi_set_named_property, napi_value, napi_create_double, napi_get_typedarray_info,
    napi_status, napi_get_element, napi_get_value_double, napi_get_array_length, napi_create_array,
    napi_set_element, napi_is_array, napi_throw_type_error
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

fn array_for_each (env: napi_env, napi_val: napi_value, func: fn(napi_env, usize, napi_value) -> ()) {
    let mut length: u32 = 0;
    let status: napi_status = unsafe { napi_get_array_length(env, napi_val, &mut length as *mut u32) };
    assert_eq!(status, napi_status::napi_ok);

    for i in 0..length {
        // create pointer to get value
        let mut result_buffer: [napi_value; 1] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        let status: napi_status = unsafe { napi_get_element(env, napi_val, i, result_buffer.as_mut_ptr()) };
        assert_eq!(status, napi_status::napi_ok);
        func(env, i as usize, result_buffer[0]);
    }
}

fn array_map (env: napi_env, napi_val: napi_value, func: fn(napi_env, usize, napi_value) -> napi_value) -> napi_value {
    let mut length: u32 = 0;
    let status: napi_status = unsafe { napi_get_array_length(env, napi_val, &mut length as *mut u32) };
    assert_eq!(status, napi_status::napi_ok);

    // let results: Vec<napi_value> = Vec::with_capacity(length as usize);

    let mut result_buffer: [napi_value; 1] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let result_status: napi_status = unsafe { napi_create_array(env, result_buffer.as_mut_ptr()) };
    assert_eq!(result_status, napi_status::napi_ok);
    let result_arr = result_buffer[0];


    for i in 0..length {
        // create pointer to get value
        let mut result: [napi_value; 1] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        let status: napi_status = unsafe { napi_get_element(env, napi_val, i, result.as_mut_ptr()) };
        assert_eq!(status, napi_status::napi_ok);

        let val = func(env, i as usize, result[0]);

        let set_status: napi_status = unsafe { napi_set_element(env, result_arr, i, val) };
        assert_eq!(set_status, napi_status::napi_ok);
    }

    result_arr
}

fn print_value (env: napi_env, i: usize, napi_val: napi_value) {
    // convert to a double
    let mut val = 0 as f64;
    let status: napi_status = unsafe { napi_get_value_double(env, napi_val, &mut val) };
    assert_eq!(status, napi_status::napi_ok);
    println!("arr[{}] {:?}", i, val);
}

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
    let arr: napi_value = buffer[0];

    array_for_each(env, arr, print_value);

    // creating the return value
    let mut local: napi_value = std::mem::zeroed();
    napi_get_undefined(env, &mut local);

    local
}

// takes a napi_u32_array and returns the sum as a napi_double
fn sum_u32_array (env: napi_env, _i: usize, napi_val: napi_value) -> napi_value {
    // creating a buffer where data can be placed
    let mut ve: Vec<u32> = Vec::new();
    let mut raw = ve.as_mut_ptr();
    let mut_ref: &mut *mut u32 = &mut raw;
    let raw_ptr: *mut *mut _ = mut_ref as *mut *mut _;

    let mut length: usize = 0;
    let mut arr_type: napi_typedarray_type = unsafe { std::mem::zeroed() };

    // getting length by passing null buffer
    let type_status = unsafe { napi_get_typedarray_info(
        env,
        napi_val,
        &mut arr_type,
        &mut length,
        raw_ptr as *mut *mut c_void,
        std::ptr::null_mut(),
        std::ptr::null_mut()
    ) };

    assert_eq!(arr_type, napi_uint32_array);
    assert_eq!(type_status, napi_status::napi_ok);

    let s: Vec<u32> = unsafe { Vec::from_raw_parts(raw, length, length) };

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
    let mut local: napi_value = unsafe { std::mem::zeroed() };
    let double_status = unsafe { napi_create_double(env, sum as f64, &mut local) };
    assert_eq!(double_status, napi_status::napi_ok);

    // returning the result
    local
}

pub extern "C" fn sum_u32_array_2d(env: napi_env, info: napi_callback_info) -> napi_value {
    let napi_arr: napi_value = get_single_param(env, info);

    let mut is_array = false;

    let status = unsafe { napi_is_array(env, napi_arr, &mut is_array) };
    assert_eq!(status, napi_status::napi_ok);

    if is_array  {
        array_map(env, napi_arr, sum_u32_array)
    } else {
        // example of error handling
        unsafe { napi_throw_type_error(env, std::ptr::null_mut(), CString::new("Expected argument to be an Array").unwrap().as_ptr()) };
        let mut local: napi_value = unsafe { std::mem::zeroed() };
        unsafe { napi_get_undefined(env, &mut local) };
        local
    }
}

// convenience wrapper to get single parameter
fn get_single_param (env: napi_env, info: napi_callback_info) -> napi_value {
    // creating a buffer of arguments
    let mut buffer: [napi_value; 1] = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let mut argc = 1 as usize;
    // getting arguments
    let status = unsafe { napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ) };

    assert_eq!(status, napi_status::napi_ok);

    buffer[0]
}

pub extern "C" fn sum_u32_array_top(env: napi_env, info: napi_callback_info) -> napi_value {
    let napi_u32_arr = get_single_param(env, info);
    sum_u32_array(env, 0, napi_u32_arr)
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
    let p3 = CString::new("sumBuffer2d").expect("CString::new failed");

    let mut local1: napi_value = std::mem::zeroed();
    let mut local2: napi_value = std::mem::zeroed();
    let mut local3: napi_value = std::mem::zeroed();

    napi_create_function(
        env,
        p1.as_ptr(),
        5,
        Some(sum_u32_array_top),
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
    napi_create_function(
        env,
        p3.as_ptr(),
        5,
        Some(sum_u32_array_2d),
        std::ptr::null_mut(),
        &mut local3,
    );
    napi_set_named_property(env, exports, p1.as_ptr(), local1);
    napi_set_named_property(env, exports, p2.as_ptr(), local2);
    napi_set_named_property(env, exports, p3.as_ptr(), local3);
    exports
}
