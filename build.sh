#!/bin/sh

cd memory_safety
cargo build --release
cd ..

cd unsafe_code
g++ main.cpp -L../memory_safety/target/release -lmemory_safety -o ../safe_cpp
