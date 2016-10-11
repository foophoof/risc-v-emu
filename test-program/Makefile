# Copyright 2016 risc-v-emu Developers
#
# Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
# http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
# http://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.

GCC ?= riscv32-unknown-elf-gcc
GCCFLAGS ?= -nostdlib -O0

test: test.c syscalls.c linker.ld
	$(GCC) $(GCCFLAGS) -T linker.ld -o $@ test.c syscalls.c

.PHONY: clean
clean:
	rm test
