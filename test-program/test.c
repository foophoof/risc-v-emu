#include "syscalls.h"
// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

int strlen(char *buf) {
    int len = 0;
    while (*buf != 0) {
        len++;
        buf++;
    }

    return len;
}

void print(char *output) {
    write(0, output, strlen(output));
}

void _start() {
    print("Hello, world!\n");
}
