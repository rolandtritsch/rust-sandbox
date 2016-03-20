extern crate libc;

use libc::{c_void, c_int};

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,
    // much more actually in here for practical JNI code, but not
    // relevant for this very simple example...
}

pub type JNIEnv = *const JNINativeInterface;

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Java_Adder_add(jre: *mut JNIEnv, class: *const c_void, v1: c_int, v2: c_int) -> c_int {
    Java_Adder_00024_add(jre, class, v1, v2)
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn Java_Adder_00024_add(jre: *mut JNIEnv, class: *const c_void, v1: c_int, v2: c_int) -> c_int {
    v1 + v2
}

#[cfg(test)]
mod test {
    use std::ptr;
    use ::Java_Adder_add;

    #[test]
    fn add_test() {
        assert_eq!(10, Java_Adder_add(ptr::null_mut(), ptr::null(), 8, 2));
    }
}
