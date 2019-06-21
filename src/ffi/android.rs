#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Priority {
    UNKNOWN   = 0,
    DEFAULT   = 1,
    VERBOSE   = 2,
    DEBUG     = 3,
    INFO      = 4,
    WARN      = 5,
    ERROR     = 6,
    FATAL     = 7,
    SILENT    = 8,
}

#[allow(non_camel_case_types)] type c_char = u8;
#[allow(non_camel_case_types)] type c_int = i32;

extern {
    pub fn __android_log_print(priority: Priority, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
    pub fn __android_log_write(priority: Priority, tag: *const c_char, text: *const c_char) -> c_int;
}
