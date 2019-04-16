## Teensy teensy 3.6 (Cortex M4 MK66FX1M0) rust embeded toolchain template

What is this? This is a template for starting an embeded project with Rust on the Teensy3.6 or other Cortex M4 MK66FX1M0 board. It can be re-configured for just about any Cortex M board such as M3 or M7 if you change the memory mapped regions (in layout.ld) from the spec in it's manual datasheet.

This project is based on a tutorial blog post [here](https://branan.github.io/teensy/2017/01/12/bootup.html), with updated utilities, newers api functions, and memory mapped for the Teensy3.6.

Another repositiory that looks useful as well: [Cortex-M Quickstart](https://github.com/rust-embedded/cortex-m-quickstart)

Some Spec Sheets:

- [Teeny3.6 MK66FX1M0 Spec](https://www.pjrc.com/teensy/K66P144M180SF5RMV2.pdf)
- [Teensy3.2 MK20DX256 Spec](https://www.pjrc.com/teensy/K20P64M72SF1RM.pdf)

Also `thumbv7em-none-eabihf` in .cargo/config should be `thumbv7em-none-eabi` if your device does not have a floating point unit (FPU) on cortex M4 or M7, `thumbv6m-none-eabi` for Cortex-M0 or M0+, and `thumbv7m-none-eabi` for Cortex-M3.

## Setting up Build Environment

Intall the nightly last shown as stable for `rls` and `clippy` on [rust-toolstate](https://rust-lang-nursery.github.io/rust-toolstate/)

**Note:**

- Make sure to replace the `2018-04-15` date below with the latest stable version you found above
- Also replace `2018-04-15` in the `rust-toolchain` file
- Make sure to replace `thumbv7em-none-eabihf` with the aplicable embeded toolchain for your device

*Install toolchain:*

```shell
$ rustup toolchain install nightly-2018-04-15
$ rustup component add --target thumbv7em-none-eabihf rust-std --toolchain=nightly-2018-04-15
```

*then install binutils*

on Linux (with pacman utils):

```shell
$ sudo pacman -S arm-none-eabihf-binutils
```

on Mac (with Homebrew):
[osx-cross/homebrew-arm](https://github.com/osx-cross/homebrew-arm)

```shell
# to tap the repository
$ brew tap osx-cross/arm
# to install the toolchain
$ brew install arm-gcc-bin
```

***Finally,*** *get the teensy loader [here](https://www.pjrc.com/teensy/loader_cli.html)* and add it somewhere in your path.