#!/usr/bin/bash

set -xe

cargo build
(cd sample && clang-18 main.c -emit-llvm -S)
opt-18 --load-pass-plugin=target/debug/libinkwell_callsite_issue.so -passes=custom-pass sample/main.ll --disable-output
