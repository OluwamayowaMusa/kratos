#!/usr/bin/env bash

set -x # Enable xtrace
set -eo pipefail # Cause script to exit on error

cargo build
echo "Kernel Built"

file='./target/target/debug/kratos'

grub-file --is-x86-multiboot $file
echo "Multiboot kernel confirmed"

mkdir -p isodir/boot/grub
cp $file isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o myos.iso isodir
