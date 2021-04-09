# faf
FaF is a Linux webserver written in Rust. It has a single goal: to demonstrate the upper bound of possible single-node performance. It is meant as a living reference project and may have cutting edge dependencies. Being a reference project, documentation and simplicity are essential and will be maintained to the best of my ability.

[FaF Repo](https://github.com/errantmind/faf)


## Build

To compile, FaF requires the following:
* linux x86_64
* nightly Rust
* [clang-12 and lld-12](https://apt.llvm.org/) to be installed and available in PATH. The version (i.e. 12) may change over time as Rust updates its LLVM version

Build and run FaF with the following commands. Please note you will need to have clang-12
```
# Build
RUSTFLAGS="-Ctarget-cpu=native -Clinker=/usr/bin/clang-12 -Clink-arg=-fuse-ld=lld-12 -Clink-arg=-flto=thin \
   -Clto=thin -Cembed-bitcode=yes -Copt-level=3 -Ccodegen-units=1 -Cforce-frame-pointers=n  \
   cargo build --verbose --release && strip --strip-all target/release/faf-ex

# Run
./target/release/faf-ex
```

You don't need all the files in this repo to compile, just the `src` directory and `Cargo.toml`. The other files are there for other purposes