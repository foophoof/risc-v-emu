# risc-v-emu

[![Build Status](https://travis-ci.org/foophoof/risc-v-emu.svg?branch=master)](https://travis-ci.org/foophoof/risc-v-emu)

**Note**: This project is primarily a learning project. You probably shouldn't
use it in any "real" code, as it may have bugs and/or security issues. Hopefully
it's interesting to read through if you're interested in RISC-V, though.

risc-v-emu is a RISC-V emulator written in Rust.

At the moment, the RV32G (IMAFD) ISA is being implemented according to the 2.1
spec, and the 1.9 privileged spec draft as of October 8th 2016 is being used for
the privileged parts.

## Usage

You need a RISC-V toolchain to compile the test program, but then you can run a
simple Hello, world program like this:

    $ make -C test-program
    $ cargo run ./test-program/test

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
