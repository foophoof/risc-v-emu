#include "syscalls.h"

#define syscall() asm("ecall")

void write_syscall(int call, int output, const void *buf, unsigned int nbytes) {
    syscall();
}

void write(int output, const void *buf, unsigned int nbytes) {
    write_syscall(0, output, buf, nbytes);
}
