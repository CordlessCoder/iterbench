#!/bin/sh
CXX='clang++' RUSTFLAGS="-Ctarget-cpu=x86-64-v2" cargo bench

