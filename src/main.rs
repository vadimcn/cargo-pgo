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
                        Some(ref s) if s == "build" => instrumented("build", true, args),
                        Some(ref s) if s == "rustc" => instrumented("rustc", true, args),
                        Some(ref s) if s == "run" => instrumented("run", true, args),
                        Some(ref s) if s == "test" => instrumented("test", true, args),
                        Some(ref s) if s == "bench" => instrumented("bench", false, args),
                        Some(ref s) => invalid_arg(s),
                        _ => usage_and_exit(),
                    }
                }
                Some(ref s) if s == "opt" => {
                    match args.next() {
                        Some(ref s) if s == "build" => optimized("build", true, args),
                        Some(ref s) if s == "rustc" => optimized("rustc", true, args),
                        Some(ref s) if s == "run" => optimized("run", true, args),
                        Some(ref s) if s == "test" => optimized("test", true, args),
                        Some(ref s) if s == "bench" => optimized("bench", false, args),
                        Some(ref s) => invalid_arg(s),
                        _ => usage_and_exit(),
                    }
                }
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
              \n    instr build|rustc ...    - build an instrumented binary\
              \n    instr run|test|bench ... - run the instrumented binary while recording profiling data\
              \n    merge                    - merge raw profiling data using llvm-profdata\
              \n    opt build|rustc ...      - merge raw profiling data, then build an optimized binary\
              \n    opt run|test|bench ...   - run the optimized binary\
              \n    clean                    - remove recorded profiling data");
    std::process::exit(1);
}

fn invalid_arg(arg: &str) {
    println!("Unexpected argument: {}\n", arg);
    usage_and_exit();
}

fn instrumented(subcommand: &str, release_flag: bool, args: env::Args) {
    let mut args: Vec<String> = args.collect();
    if release_flag {
        args.insert(0, "--release".to_string());
    }
    let rustflags = format!("--cfg=profiling -Cllvm-args=-profile-generate=target/release/pgo/%p.profraw -Lnative={0} -Clink-args=-lprofiler-rt",
        env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
    let mut child = Command::new("cargo")
        .arg(subcommand)
        .args(&args)
        .env("RUSTFLAGS", rustflags)
        .spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));
    std::process::exit(exit_status.code().unwrap_or(-1));
}

fn merge_profiles() -> bool {
    let dir = match fs::read_dir("target/release/pgo") {
        Ok(dir) => dir,
        Err(_) => return false,
    };
    let mut raw_profiles = Vec::<std::path::PathBuf>::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > 0 && ext == "profraw" {
                        raw_profiles.push(entry.path());
                    }
                }
            }
        }
    }
    if raw_profiles.len() == 0 {
        return false;
    }
    Command::new("llvm-profdata")
        .arg("merge")
        .args(&raw_profiles)
        .arg("--output").arg("target/release/pgo/pgo.profdata")
        .output().expect("failed to execute llvm-profdata");
    return true;
}

fn optimized(subcommand: &str, release_flag: bool, args: env::Args) {
    let mut args = args.collect::<Vec<_>>();
    if release_flag {
        args.insert(0, "--release".to_string());
    }
    if !merge_profiles() {
        println!("Warning: no recorded profiling data was found.");
    }

    let rustflags = "-Cllvm-args=-profile-use=target/release/pgo/pgo.profdata";
    let mut child = Command::new("cargo")
        .arg(subcommand)
        .args(&args)
        .env("RUSTFLAGS", rustflags)
        .spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));
    std::process::exit(exit_status.code().unwrap_or(-1)); 
}

fn clean() {
    let _ = fs::remove_dir_all("target/release/pgo");
}
