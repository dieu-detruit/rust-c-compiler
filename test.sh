#!/usr/bin/zsh

echo input: ./input/main.c
cat ./input/main.c
cc -c -o ./input/print.o ./input/print.c
cargo run -- "$(cat ./input/main.c)" > ./input/out.S
cc -o a.out ./input/out.S ./input/print.o
./a.out

echo output: $?
