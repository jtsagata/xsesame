#!/usr/bin/env bash

rm ./target/debug/xsesame
cargo build

echo "help"
./target/debug/xsesame list --help

echo "Test with no args "
LANG=el_GR.urf8 ./target/debug/xsesame list

echo "Test with no args (pipe)"
LANG=el_GR.urf8 ./target/debug/xsesame list -e=plain | cat | head -5

echo "Test emoji and nls all"
LANG=el_GR.urf8 ./target/debug/xsesame list  -l -w=all


echo "Test emoji and nls only valid"
LANG=el_GR.urf8 ./target/debug/xsesame list -e=check -l -w=valid -c=show

echo "Test empty"
LANG=el_GR.urf8 ./target/debug/xsesame list -d test
