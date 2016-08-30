# Profile-Guided Optimization for Rust

A Cargo-based workflow for Profile-Guilded Optimization (PGO).

## Setup
- Clone this repo,
- `git submodule update --init`.  
- `cargo build --release`
- Add `target/release` to your PATH.

## Usage

### Remove any stale profiling data
`cargo pgo clean`

### Instrument your binary for profiling
`cargo pgo instr build`  

This will spawn a normal Cargo build (with some extra flags passed to rustc via RUSTFLAGS), so all 
the usual `cargo build` flags apply.
Note that Cargo-PGO will automatically add the `--release` flag, since there's very reason to 
PGO-optimize debug builds.

### Run training scenarios
`cargo pgo instr run <params1>`  
`cargo pgo instr run <params2>`  
etc...  
You can also use `cargo pgo instr test` or `cargo pgo instr bench`.
Each execution will create a new raw profile file under `target/release/pgo`.

### Merge profiles
In order to use profiling data in build, raw profiles must be first merged
into an 'indexed' format:  
`cargo pgo merge`  
This will create the `target/release/pgo.profdata` file.

### Build an optimized binary
`cargo pgo opt build`

### Run your optimized binary
`cargo pgo opt run|test|bench`

("Why not just 'cargo run'?" - you ask.  This is because Cargo keeps track of flags passed to rustc, 
and automatically rebuilds the target if they change.  Thus, `cargo run` would revert your 
binary back to non-optimized state).

### Do it quicker!
Because Cargo will automatically [re]build stale binaries before running them, you can skip both 
build steps above and jump straight to running!  In addition to that, `cargo pgo opt ...` commands 
will automatically merge raw profiles if needed.  Therefore, the above workflow can be accomplished
in just two steps:  

`cargo pgo instr run`  
`cargo pgo opt run`
