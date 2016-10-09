#include "syscalls.h"

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
