#!/usr/bin/env bash

set -x # Enable xtrace
set -eo pipefail # Cause script to exit on error

i686-elf-as boot.s -o boot.o
echo "Assemble done"

i686-elf-gcc -c kernel.c -o kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
echo "Kernel compiled"

i686-elf-gcc -T linker.ld -o myos.bin -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc
echo "Kernel linked"

grub-file --is-x86-multiboot myos.bin
echo "Multiboot kernel confirmed"

mkdir -p isodir/boot/grub
cp myos.bin isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o myos.iso isodir
