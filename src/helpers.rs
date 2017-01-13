use libc::c_char;
use std::ffi::CStr;
use std::str::from_utf8;

pub fn from_c_char_to_string(char_string: *const c_char) -> String {
    if char_string.is_null() {
        return String::new();
    }

    let c_value_string = unsafe { CStr::from_ptr(char_string) };

    from_utf8(c_value_string.to_bytes()).unwrap().to_owned()
}
