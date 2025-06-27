#![allow(clippy::missing_safety_doc)] // lol
#![allow(clippy::missing_panics_doc)]

pub mod mandelbrot;
pub mod thing;

use std::ffi::{CString, c_char};

#[unsafe(no_mangle)]
pub extern "C" fn get_string_from_rust() -> *const c_char {
    let s = CString::new("Hello World... memory safety guaranteed by rust.").expect("How");

    s.into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_rust_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}
