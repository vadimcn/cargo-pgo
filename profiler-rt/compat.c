void __llvm_profile_set_filename(const char *Name);

void __llvm_profile_override_default_filename(const char *Name) {
    __llvm_profile_set_filename(Name);
}
