void __llvm_profile_set_filename(const char *Name);
void __llvm_profile_register_write_file_atexit();
void __llvm_profile_initialize_file();

void __llvm_profile_override_default_filename(const char *Name) {
    __llvm_profile_register_write_file_atexit();
    __llvm_profile_set_filename(Name);
    __llvm_profile_initialize_file();
}
