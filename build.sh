#!/bin/bash

mkdir -p bins

cargo build -p benchrunner --release
mv target/release/benchrunner bins/benchrunner-glibc

cargo build -p benchrunner --release --features=jemalloc
mv target/release/benchrunner bins/benchrunner-jemalloc

cargo build -p benchrunner --release --features=tcmalloc
mv target/release/benchrunner bins/benchrunner-tcmalloc

cargo build -p benchrunner --release --features=hoard
mv target/release/benchrunner bins/benchrunner-hoard
