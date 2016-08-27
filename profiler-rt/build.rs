extern crate gcc;
use std::env;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("compiler-rt/lib/profile"); 
    gcc::Config::new()
                .file(dir.join("GCDAProfiling.c"))
                .file(dir.join("InstrProfiling.c"))
                .file(dir.join("InstrProfilingValue.c"))
                .file(dir.join("InstrProfilingBuffer.c"))
                .file(dir.join("InstrProfilingFile.c"))
                .file(dir.join("InstrProfilingMerge.c"))
                .file(dir.join("InstrProfilingMergeFile.c"))
                .file(dir.join("InstrProfilingWriter.c"))
                .file(dir.join("InstrProfilingPlatformDarwin.c"))
                .file(dir.join("InstrProfilingPlatformLinux.c"))
                .file(dir.join("InstrProfilingPlatformOther.c"))
                .file(dir.join("InstrProfilingUtil.c"))
                .file(dir.join("InstrProfilingRuntime.cc"))
                .include(dir)
                .compile("libcompiler-rt.profile.a");
}