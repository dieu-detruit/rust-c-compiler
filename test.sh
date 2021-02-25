#!/usr/bin/zsh

expr="-8 + 2 * (4 + 1)"
echo input: $expr
cargo run -- $expr > out.S
cc out.S
./a.out

echo output: $?
