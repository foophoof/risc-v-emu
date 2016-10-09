GCC ?= riscv64-unknown-elf-gcc
GCCFLAGS ?= -nostdlib -m32 -O0

test: test.c syscalls.c linker.ld
	$(GCC) $(GCCFLAGS) -T linker.ld -o $@ test.c syscalls.c

.PHONY: clean
clean:
	rm test
