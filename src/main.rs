mod ffi;
use std::ffi::CStr;

pub fn main() {

    let mut ptr = std::ptr::null_mut();
    unsafe {
        ffi::ocio_GetVersion(&mut ptr);
        println!("Bound OpenColorIO version is {}", CStr::from_ptr(ptr).to_string_lossy().to_string());
    }

}