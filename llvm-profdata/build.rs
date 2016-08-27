extern crate cmake;

fn main() {
    cmake::Config::new("llvm")
                 .profile("Release")
                 .build_target("llvm-profdata")
                 .build();
}
