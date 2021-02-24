#!/usr/bin/zsh

expr="1 + 2 * (6 - 4) - 4"
cargo run -- $expr > out.S | tail -f
cc out.S
./a.out

echo input: $expr
echo output: $?
