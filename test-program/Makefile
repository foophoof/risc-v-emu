GCC ?= riscv32-unknown-elf-gcc
GCCFLAGS ?= -nostdlib -O0

test: test.c syscalls.c linker.ld
	$(GCC) $(GCCFLAGS) -T linker.ld -o $@ test.c syscalls.c

.PHONY: clean
clean:
	rm test
