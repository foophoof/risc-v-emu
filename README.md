# risc-v-cpu

**Note**: This project is primarily a learning project. You probably shouldn't
use it in any "real" code, as it may have bugs and/or security issues. Hopefully
it's interesting to read through if you're interested in RISC-V, though.

risc-v-cpu is a RISC-V CPU emulator written in Rust.

At the moment, only the RV32G ISA is supported, but support for other ISAs or
extensions may be added in the future.

## Usage

You need a RISC-V toolchain to compile the test program, but then you can run a
simple Hello, world program like this:

    $ make -C test-program
    $ cargo run ./test-program/test
