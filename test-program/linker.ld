/*
Copyright 2016 risc-v-emu Developers

Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
http://opensource.org/licenses/MIT>, at your option. This file may not be
copied, modified, or distributed except according to those terms.
*/

ENTRY(main)

SECTIONS
{
    . = 64K;

    .text BLOCK(4K) : ALIGN(4K)
	{
		*(.text)
	}

    /* Read-only data. */
	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	}

	/* Read-write data (uninitialized) and stack */
	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}

    .comment BLOCK(4K) : ALIGN(4K)
    {
        *(.comment)
    }

    .shstrtab BLOCK(4K) : ALIGN(4K)
    {
        *(.shstrtab)
    }

    .symtab BLOCK(4K) : ALIGN(4K)
    {
        *(.symtab)
    }

    .strtab BLOCK(4K) : ALIGN(4K)
    {
        *(.strtab)
    }
}
