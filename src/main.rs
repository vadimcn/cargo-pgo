use std::process::Command;
use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next().expect("program name");
    match args.next() {
        Some(ref s) if s == "pgo" => {
            match args.next() {
                Some(ref s) if s == "instr" => {
                    match args.next() {
                        Some(ref s) if s == "build" => instrumented("build", args),
                        Some(ref s) if s == "run" => instrumented("run", args),
                        Some(ref s) if s == "test" => instrumented("test", args),
                        Some(ref s) if s == "bench" => instrumented("bench", args),
                        Some(ref s) => invalid_arg(s),
                        _ => usage_and_exit(),
                    }
                }
                Some(ref s) if s == "build" => optimized("build", args),
                Some(ref s) if s == "run" => optimized("run", args),
                Some(ref s) if s == "test" => optimized("test", args),
                Some(ref s) if s == "bench" => optimized("bench", args),
                Some(ref s) if s == "merge" => { merge_profiles(); }
                Some(ref s) if s == "clean" => clean(),
                Some(ref s) => invalid_arg(s),
                _ => usage_and_exit(),
            }
        }
        Some(ref s) => invalid_arg(s),
        _ => usage_and_exit(),
    }
}

fn usage_and_exit() {
    println!("Usage: cargo pgo <command>...\
              \nCommands:\
              \n    instr build ...          - build instrumented binary\
              \n    instr run|test|bench ... - run instrumented binary while recording profiling data\
              \n    merge                    - merge raw profiling data using llvm-profdata\
              \n    build ...                - merge raw profiling data, then build optimized binary\
              \n    run|test|bench ...       - run optimized binary\
              \n    clean                    - remove recorded profiling data");
    std::process::exit(1);
}

fn invalid_arg(arg: &str) {
    println!("Unexpected argument: {}\n", arg);
    usage_and_exit();
}

fn instrumented(subcommand: &str, args: env::Args) {
    let args = args.collect::<Vec<_>>();
    let mut child = Command::new("cargo")
        .arg(subcommand)
        .arg("--release")
        .args(&args)
        .env("RUSTFLAGS", get_instr_rustflags())
        .env("LLVM_PROFILE_FILE", "target/release/pgo/raw/%p.profraw")
        .spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));
    std::process::exit(exit_status.code().unwrap_or(-1));
}

fn merge_profiles() -> bool {
    let dir = match fs::read_dir("target/release/pgo/raw") {
        Ok(dir) => dir,
        Err(_) => return false,
    };
    let files: Vec<_> = dir.flat_map(|x| x.map(|x| x.path())).collect();
    if files.len() == 0 {
        return false;
    }
    Command::new("llvm-profdata")
        .arg("merge")
        .args(&files)
        .arg("--output").arg("target/release/pgo/pgo.profdata")
        .output().expect("failed to execute llvm-profdata");
    return true;
}

fn optimized(subcommand: &str, args: env::Args) {
    let args = args.collect::<Vec<_>>();
    if !merge_profiles() {
        println!("Warning: no recorded profiling data was found.");
    }

    let mut child = Command::new("cargo")
        .arg(subcommand)
        .arg("--release")
        .args(&args)
        .env("RUSTFLAGS", format!("-Cpasses=pgo-instr-use -Cllvm-args=-pgo-test-profile-file={}",
                                    "target/release/pgo/pgo.profdata"))
        .spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));
    std::process::exit(exit_status.code().unwrap_or(-1)); 
}

fn clean() {
    let _ = fs::remove_dir_all("target/release/pgo");
}

fn get_instr_rustflags() -> String {
    format!("--cfg=profiling -Cpasses=pgo-instr-gen -Cpasses=instrprof -L{}",
        env::current_exe().unwrap().parent().unwrap().to_str().unwrap()) 
}
