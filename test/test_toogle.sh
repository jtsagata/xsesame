#!/usr/bin/env bash

rm ./target/debug/xsesame
cargo build

pushd test
tar xvfz samples.tar.gz
popd

printf "\n** Test with no args \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame disable

printf "\n** Test with bad name \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame disable lala

printf "\n** Test with bad session \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame disable --what=all invalid_parse

printf "\n** Before list \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame list

printf "\n** disable some \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame disable pop
LANG=el_GR.urf8 ./target/debug/xsesame disable lxde
LANG=el_GR.urf8 ./target/debug/xsesame disable plasma
LANG=el_GR.urf8 ./target/debug/xsesame disable pop
LANG=el_GR.urf8 ./target/debug/xsesame disable budgie-desktop
LANG=el_GR.urf8 ./target/debug/xsesame disable cinnamon
LANG=el_GR.urf8 ./target/debug/xsesame disable cinnamon2d -J
LANG=el_GR.urf8 ./target/debug/xsesame disable invalid_noname -J
LANG=el_GR.urf8 ./target/debug/xsesame disable icewm-session

printf "\n** After list \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame list

printf "\n** enable some \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame enable pop
LANG=el_GR.urf8 ./target/debug/xsesame enable pop
LANG=el_GR.urf8 ./target/debug/xsesame enable lxde
LANG=el_GR.urf8 ./target/debug/xsesame enable plasma
LANG=el_GR.urf8 ./target/debug/xsesame enable pop
LANG=el_GR.urf8 ./target/debug/xsesame enable budgie-desktop

printf "\n** After list \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame list

printf "\n** toggle some \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame toggle pop
LANG=el_GR.urf8 ./target/debug/xsesame toggle cinnamon

printf "\n** After list \n\n"
LANG=el_GR.urf8 ./target/debug/xsesame list
