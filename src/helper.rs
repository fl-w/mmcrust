pub fn cstr_to_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() }
}
