# faf-example

This project demonstrates how to use FaF

FaF is a Linux webserver written in Rust. It has a single goal: to demonstrate the upper bound of possible single-node performance. It is meant as a living reference project and may have cutting edge dependencies. Being a reference project, documentation and simplicity are essential and will be maintained to the best of my ability.

[FaF Repo](https://github.com/errantmind/faf)


## Build

To compile, FaF requires the following:
* linux x86_64
* nightly Rust
* [clang-15 and lld-15](https://apt.llvm.org/) to be installed and available in PATH. The version (i.e. 15) may change over time as Rust updates its LLVM version

Build and run FaF with the following commands. (Note, not all these flags are strictly necessary, they are there to ensure future performance. Remove any that cause your project issues)
```
# Build
RUSTFLAGS="-Ctarget-cpu=native -Ztune-cpu=native -Zmutable-noalias=yes -Clink-arg=-fexperimental-new-pass-manager \
   -Clinker=/usr/bin/clang-15 -Clink-arg=-fuse-ld=/usr/bin/ld.lld-15 -Clink-arg=-flto=thin -Clto=thin -Copt-level=3 \
   -Ccodegen-units=1 -Cpanic=abort -Cembed-bitcode=yes -Cforce-frame-pointers=n -Cdebug-assertions=no -Coverflow-checks=no \
   -Ccontrol-flow-guard=no -Clink-dead-code=no -Zno-parallel-llvm" \
   /root/.cargo/bin/cargo build --release --target x86_64-unknown-linux-gnu -Zbuild-std=panic_abort,core,std,alloc,proc_macro,compiler_builtins \
   && strip ./target/x86_64-unknown-linux-gnu/release/faf-ex

# Run
./target/x86_64-unknown-linux-gnu/release/faf-ex
```

You don't need all the files in this repo to compile, just the `src` directory and `Cargo.toml`. The other files are there for other purposes

## Related Crates

* [faf-syscall](https://github.com/errantmind/faf-syscall)
* [faf-http-date](https://github.com/errantmind/faf-http-date)
