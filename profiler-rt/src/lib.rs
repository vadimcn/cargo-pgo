
extern "C" {
    fn __llvm_profile_register_write_file_atexit();
    fn __llvm_profile_initialize_file();
}

pub fn initialize() {
    unsafe {
        __llvm_profile_register_write_file_atexit();
        __llvm_profile_initialize_file();
    }
}
