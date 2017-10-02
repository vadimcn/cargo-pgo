extern crate cmake;
extern crate gcc;
use std::path::PathBuf;
use std::env;

fn main() {
    cmake::Config::new("llvm")
        .define("LLVM_ENABLE_ZLIB", "OFF")
        .define("LLVM_INCLUDE_TESTS", "OFF")
        .profile("Release")
        .build_target("llvm-profdata")
        .build();

    let out_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
    gcc::Config::new()
                .file("src/profdata.cpp")
                .cpp(true)
                .flag("-std=c++11")
                .flag("-fno-exceptions")
                .flag("-fno-rtti")
                .define("NDEBUG", None)
                .opt_level(2)
                .include("llvm/include")
                .include(out_dir.join("build/include"))
                .compile("libprofdataimpl.a");

    println!("cargo:rustc-link-search={}", out_dir.join("build/lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=LLVMProfileData");
    println!("cargo:rustc-link-lib=LLVMCore");
    println!("cargo:rustc-link-lib=LLVMSupport");
    println!("cargo:rustc-link-lib=curses");
}
