// Copyright 2016 risc-v-emulator Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

int factorial(int input) {
    if (input <= 1) {
        return 1;
    }

    return input * factorial(input - 1);
}

int main() {
    return factorial(7);
}
