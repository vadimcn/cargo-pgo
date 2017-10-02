# Profile-Guided Optimization workflow for Cargo

## Setup

- `git clone https://github.com/vadimcn/cargo-pgo.git`,
- `cd cargo-pgo`,
- `git submodule update --init` - this may take a while as LLVM is one of the upstream 
    dependencies.  Fortunately, only a small part of it needs to be built.
- `cargo build --release`,
- Add `cargo-pgo/target/release` to your PATH.

## Usage

### Remove any old profiling data
```
cargo pgo clean
```

### Instrument your binary for profiling
```
cargo pgo instr build
```  

This will spawn a normal Cargo build (with some extra flags passed to rustc via RUSTFLAGS), so all 
the usual `cargo build` flags do apply.
Note that cargo-pgo will automatically add the `--release` flag, since there's little reason to 
PGO-optimize debug builds.

### Run training scenarios
```
cargo pgo instr run <params1>
cargo pgo instr run <params2>
...  
```
You can also use `cargo pgo instr test` or `cargo pgo instr bench`.  
Each execution will create a new raw profile file under `target/release/pgo`.

### Merge profiles
Before using generated profiles, they must be first merged into an 'indexed' format:  
```
cargo pgo merge
``` 
The output will be saved in `target/release/pgo/merged.profdata`.

### Build optimized binary
```
cargo pgo opt build
```

### Run optimized binary
```
cargo pgo opt run|test|bench
```

("Why not just '`cargo run`'?": Cargo keeps track of the flags it had passed 
to rustc last time, and automatically rebuilds the target if they change.  Thus, `cargo run` 
would first revert the binary back to non-optimized state, which probably isn't what you want.)

### Do it in less steps
Cargo automatically (re)builds stale binaries before running them, so you may skip both of the
build steps above and jump straight to running.  In addition to that, `cargo pgo opt ...` commands 
will automatically merge raw profiles if needed.
All of the above steps may be condensed to just two commands:
```
cargo pgo instr run
cargo pgo opt run
```
