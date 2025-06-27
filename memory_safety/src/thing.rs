use std::ffi::{CString, c_char};

#[repr(C)]
#[derive(Debug)]
pub struct Thing {
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: *const c_char,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_thing() -> *mut Thing {
    Box::into_raw(Box::new(Thing {
        int_value: 42,
        float_value: std::f32::consts::PI,
        string_value: c"Hello c".as_ptr(),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_thing(thing: *mut Thing) {
    if !thing.is_null() {
        unsafe {
            println!(">> Destroying thing");
            drop(Box::from_raw(thing));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_set_int_value(thing: *mut Thing, value: i32) {
    unsafe {
        if thing.is_null() {
            println!(">> thing_set_int_value: thing was null");
        } else {
            (*thing).int_value = value;
            println!(">> thing_set_int_value: int_value set to {value}");
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_set_float_value(thing: *mut Thing, value: f32) {
    unsafe {
        if thing.is_null() {
            println!(">> thing_set_float_value: thing was null");
        } else {
            (*thing).float_value = value;
            println!(">> thing_set_float_value: float_value set to {value}");
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_set_string_value(thing: *mut Thing, value: *const c_char) {
    unsafe {
        if thing.is_null() {
            println!(">> thing_set_string_value: thing was null");
        } else {
            (*thing).string_value = value;
            println!(
                ">> thing_set_string_value: string_value set to {:?}",
                std::ffi::CStr::from_ptr(value)
            );
        }
    }
}

// getters

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_get_int_value(thing: *const Thing) -> i32 {
    if thing.is_null() {
        println!(">> thing_get_int_value: thing was null");
        return 0;
    }
    unsafe { (*thing).int_value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_get_float_value(thing: *const Thing) -> f32 {
    if thing.is_null() {
        println!(">> thing_get_float_value: thing was null");
        return 0.0;
    }

    unsafe { (*thing).float_value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_get_string_value(thing: *const Thing) -> *const c_char {
    if thing.is_null() {
        println!(">> thing_get_string_value: thing was null");
        return std::ptr::null();
    }

    unsafe { (*thing).string_value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn thing_print(thing: *const Thing) {
    if thing.is_null() {
        println!(">> thing_print: thing was null");
        return;
    }

    unsafe {
        println!(">> {:#?}", *thing);
    }
}
