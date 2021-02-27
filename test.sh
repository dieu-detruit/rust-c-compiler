#!/usr/bin/zsh

expr="hoge = 2 * 3; fuga = 7 * 1; return hoge + fuga;"
echo input: $expr
cargo run -- $expr > out.S
cc out.S
./a.out

echo output: $?
