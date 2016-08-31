# Profile-Guided Optimization workflow for Cargo

## Setup
- Clone this repo,
- `git submodule update --init` - this may take a while as LLVM is one of the upstream 
    dependencies.  Fortunately, only a small part of it needs to be built.
- `cargo build --release`,
- Add `target/release` to your PATH.

Cargo install'ing is not supported, as it does not allow deployment of non-binary resources.

## Usage

### Remove any stale profiling data
`cargo pgo clean`

### Instrument your binary for profiling
`cargo pgo instr build`  

This will spawn a normal Cargo build (with some extra flags passed to rustc via RUSTFLAGS), so all 
the usual `cargo build` flags do apply.
Note that cargo-pgo will automatically add the `--release` flag, since there's little reason to 
PGO-optimize debug builds.

### Run training scenarios
`cargo pgo instr run <params1>`  
`cargo pgo instr run <params2>`  
etc...  
You can also use `cargo pgo instr test` or `cargo pgo instr bench`.  
Each execution will create a new raw profile file under `target/release/pgo`.

### Merge profiles
Before using generated profiles, they must be first merged into an 'indexed' format:  
`cargo pgo merge`  
This will create `target/release/pgo.profdata` file.

### Build optimized binary
`cargo pgo opt build`

### Run optimized binary
`cargo pgo opt run|test|bench`

("Why not just 'cargo run'?" - you ask.  Because Cargo keeps track of the flags it passed 
to rustc last time, and automatically rebuilds the target if they change.  Thus, `cargo run` 
would revert the binary back to non-optimized state, which probably isn't what you want.)

### Do it quicker!
Cargo will automatically [re]build stale binaries before running them, so you may skip both of the
build steps above and jump straight to running!  In addition to that, `cargo pgo opt ...` commands 
will automatically merge raw profiles if needed.  So:  

`cargo pgo instr run`  
`cargo pgo opt run`
