#!/usr/bin/zsh

echo input: ./input/main.c
cat ./input/main.c
gcc -c -o ./input/print.o ./input/print.c
cargo run -- "$(python3 commentout.py ./input/main.c)" > ./input/out.S
python3 optimize.py
gcc -g -o a.out ./input/out_optimized.S ./input/print.o
#cc -o a.out ./input/out.S ./input/print.o
./a.out

echo output: $?
