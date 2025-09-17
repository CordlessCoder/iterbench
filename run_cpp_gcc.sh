#!/bin/sh
g++ -O3 -march=x86-64-v2 -std=c++23 src/max.cxx main.cxx -o cpp_gcc && ./cpp_gcc
