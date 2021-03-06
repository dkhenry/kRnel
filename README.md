# kRnel #

A small 64-bit, Multiboot kernel written in Rust.

## Dependencies ##
#### Build ####
* `clang` (capable of cross-compiling for x86-64)
* `nasm`
* `rust` (0.8)
* `ninja`
* `xorriso`

#### Runtime ####
* a Multiboot-compliant bootloader

#### Emulation ####
* `qemu`
* `ninja`

## Build Instructions ##

#### Linux ####
Some distributions have compilers that will work just fine out-of-the-box, while some don't.

```bash
$ sudo yaourt -S clang nasm qemu rust ninja libisoburn
$ cd path/to/rustboot
$ ./configure
$ ninja
```

#### OSX ####
You should use Homebrew for as much as possible.  The configure script
autogenerates `binutils` for you because OSX has a broken version by default.
Please note that this assumes you already have `clang` installed via Xcode.

```bash
$ brew install nasm qemu ninja rust xorriso
$ cd path/to/rustboot
$ ./configure
$ ninja
```

## Emulation Instructions ##
```bash
$ ninja run
```

## Legal ##
Copyright (C) 2013 Arcterus.
All rights reserved.

See License.txt for description of this project's license.
