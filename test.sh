#!/usr/bin/zsh

echo input: input.c
cat input.c
cargo run -- "$(cat input.c)" > out.S
cc out.S
./a.out

echo output: $?
