// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#include "syscalls.h"

#define syscall() asm("ecall")

void write_syscall(int call, int output, const void *buf, unsigned int nbytes) {
    syscall();
}

void write(int output, const void *buf, unsigned int nbytes) {
    write_syscall(0, output, buf, nbytes);
}
