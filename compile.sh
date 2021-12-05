#!/bin/sh

cargo build --release --target x86_64-unknown-linux-gnu -Zbuild-std=panic_abort,core,std,alloc,proc_macro,compiler_builtins && strip ./target/x86_64-unknown-linux-gnu/release/faf-ex
