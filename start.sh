#!/bin/sh

for i in $(seq 0 $(($(nproc --all)-1))); do
  #sudo taskset -c $i
  sudo nice -19 target/x86_64-unknown-linux-gnu/release/faf-ex $i &
done

wait
