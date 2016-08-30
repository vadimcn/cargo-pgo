extern crate cmake;
extern crate gcc;
extern crate cpp;
use std::path::PathBuf;
use std::env;

fn main() {
    cmake::Config::new("llvm")
                 .profile("Release")
                 .build_target("LLVMProfileData")
                 .build();

    let out_dir = PathBuf::from(&env::var("OUT_DIR").unwrap()); 
    cpp::build("src/lib.rs", "crate_name", |cfg| {
        cfg.flag("-std=c++11")
            .include("llvm/include")
            .include(out_dir.join("build/include"));
    });    
/*
    let out_dir = PathBuf::from(&env::var("OUT_DIR").unwrap()); 
    gcc::Config::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("src/merge_instr_profiles.cpp")
        .include("llvm/include")
        .include(out_dir.join("build/include"))
        .define("main", Some("profdata_main"))
        .compile("libmerge-profiles.a");
    
    println!("cargo:rustc-link-search={}", out_dir.join("build/lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=LLVMCore");
    println!("cargo:rustc-link-lib=LLVMSupport");
    println!("cargo:rustc-link-lib=LLVMProfileData");
*/    
}
