ENTRY(_start)

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
