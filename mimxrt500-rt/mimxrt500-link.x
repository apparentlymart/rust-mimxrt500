/* # Interrupt vectors */
EXTERN(__INTERRUPTS); /* `static` variable similar to `__EXCEPTIONS` */

INCLUDE memory.x
INCLUDE device.x

/* # Entry point = reset vector */
EXTERN(__RESET_VECTOR);
EXTERN(Reset);
ENTRY(Reset);

/* # Exception vectors */
/* This is effectively weak aliasing at the linker level */
/* The user can override any of these aliases by defining the corresponding symbol themselves (cf.
   the `exception!` macro) */
EXTERN(__EXCEPTIONS); /* depends on all the these PROVIDED symbols */

EXTERN(DefaultHandler);

PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

PROVIDE(DefaultHandler = DefaultHandler_);
PROVIDE(HardFault = HardFault_);

/* # Pre-initialization function */
/* If the user overrides this using the `pre_init!` macro or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = DefaultPreInit);

/* Default stack size is 32KiB. We place the stack at the start of RAM since
   the earlier RAM banks are closer to the CPU core. */
PROVIDE(_stack_size = 32768);

/* # Sections */
SECTIONS
{
  PROVIDE(_ram_start = ORIGIN(RAM));
  PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _ram_start + _stack_size);
  _stack_end = _stack_start - _stack_size; /* NOTE: Grows downwards */

  /* ## Sections in FLASH */
  /* ### Flash configuration header for the RT5xx boot ROM */
  .boot_hdr ORIGIN(FLASH) :
  {
    FILL(0x00000000)
    . = ORIGIN(FLASH) + 0x0400;
    _boot_hdr = .;
    KEEP(*(.boot_hdr .boot_hdr.*));
    . = ORIGIN(FLASH) + 0x1000;
    FILL(0xffffffff)
  } > FLASH

  /* ### Vector table */
  .vector_table (ORIGIN(FLASH) + 0x1000) :
  {
    __bootrom_image_start = .;
    __vector_table = .;

    /* Initial Stack Pointer (SP) value.
     * We mask the bottom three bits to force 8-byte alignment.
     * Despite having an assert for this later, it's possible that a separate
     * linker script could override _stack_start after the assert is checked.
     */
    LONG(_stack_start & 0xFFFFFFF8);

    /* Reset vector */
    KEEP(*(.vector_table.reset_vector)); /* this is the `__RESET_VECTOR` symbol */
    __reset_vector = .;

    /* Exceptions */
    KEEP(*(.vector_table.rt500_exceptions)); /* this is the `__RT500_EXCEPTIONS` symbol */
    __eexceptions = .;

    /* Device specific interrupts */
    KEEP(*(.vector_table.interrupts)); /* this is the `__INTERRUPTS` symbol */
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  /* ### .text */
  .text _stext :
  {
    . = ALIGN(4);
    __stext = .;
    *(.Reset);

    *(.text .text.*);

    /* The HardFaultTrampoline uses the `b` instruction to enter `HardFault`,
       so must be placed close to it. */
    . = ALIGN(4);
    *(.HardFaultTrampoline);
    . = ALIGN(4);
    *(.HardFault.*);

    . = ALIGN(4);
    __sinit = .;
    *(.init .init.*);
    __einit = .;
    . = ALIGN(4);
    __sfini = .;
    *(.fini .fini.*);
    __efini = .;

    . = ALIGN(4); /* Pad .text to the alignment to workaround overlapping load section bug in old lld */
    __etext = .;
  } > FLASH

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  /* ## Sections in RAM (starting immediately after the stack) */
  /* ### .data */
  .data _stack_start : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > RAM AT>FLASH
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
   * use the .data loading mechanism by pushing __edata. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __edata = .;

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .rebootdata (data that survives a software reset) */
  .rebootdata : ALIGN(4)
  {
    . = ALIGN(4);
    __srebootdata = .;
    *(.rebootdata .rebootdata.*);
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > REBOOTRAM AT>FLASH
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
   * use the .data loading mechanism by pushing __edata. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __erebootdata = .;

  /* LMA of .data */
  __sireebootdata = LOADADDR(.rebootdata);

  /* ### .gnu.sgstubs
     This section contains the TrustZone-M veneers put there by the Arm GNU linker. */
  /* Security Attribution Unit blocks must be 32 bytes aligned. */
  /* Note that this pads the FLASH usage to 32 byte alignment. */
  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
    __bootrom_image_end = .;
  } > FLASH
  /* Place `__veneer_limit` outside the `.gnu.sgstubs` section because veneers are
   * always inserted last in the section, which would otherwise be _after_ the `__veneer_limit` symbol.
   */
  . = ALIGN(32);
  __veneer_limit = .;

  /* ### .bss */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON); /* Uninitialized C statics */
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > RAM
  /* Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
   * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
   * output region or load region in those user sections! */
  . = ALIGN(4);
  __ebss = .;

  /* ### .uninit */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  /* Place the heap right after `.uninit` in RAM */
  PROVIDE(__sheap = __euninit);

  /* ## .got */
  /* Dynamic relocations are unsupported. This section is only used to detect relocatable code in
     the input files and raise an error if relocatable code is found */
  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

__image_hdr_size = __bootrom_image_end - __bootrom_image_start;
__image_hdr_type = 0x00000000; /* Plain image */
__image_hdr_load_addr = __bootrom_image_start;

/* Do not exceed this mark in the error messages below                                    | */
/* # Alignment checks */
ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(mimxrt500-rt): the start of the FLASH region must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(mimxrt500-rt): the start of the RAM region must be 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(mimxrt500-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(mimxrt500-rt): the LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(mimxrt500-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(mimxrt500-rt): start of .heap is not 4-byte aligned");

ASSERT(_stack_start % 8 == 0, "
ERROR(mimxrt500-rt): stack start address is not 8-byte aligned.
If you have set _stack_start, check it's set to an address which is a multiple of 8 bytes.
If you haven't, stack starts at the end of RAM by default. Check that both RAM
origin and length are set to multiples of 8 in the `memory.x` file.");

/* # Position checks */

/* ## .vector_table */
ASSERT(__reset_vector == ADDR(.vector_table) + 0x8, "
BUG(mimxrt500-rt): the reset vector is missing");

ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "
BUG(mimxrt500-rt): the exception vectors are missing");

ASSERT(SIZEOF(.vector_table) > 0x40, "
ERROR(mimxrt500-rt): The interrupt vectors are missing.
Possible solutions, from most likely to less likely:
- Link to a svd2rust generated device crate
- Check that you actually use the device/hal/bsp crate in your code
- Disable the 'device' feature of cortex-m-rt to build a generic application (a dependency
may be enabling it)
- Supply the interrupt handlers yourself. Check the documentation for details.");

/* ## .text */
ASSERT(ADDR(.vector_table) + SIZEOF(.vector_table) <= _stext, "
ERROR(mimxrt500-rt): The .text section can't be placed inside the .vector_table section
Set _stext to an address greater than the end of .vector_table (See output of `nm`)");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(FLASH) + LENGTH(FLASH), "
ERROR(mimxrt500-rt): The .text section must be placed inside the FLASH memory.
Set _stext to an address smaller than 'ORIGIN(FLASH) + LENGTH(FLASH)'");

/* # Other checks */
ASSERT(SIZEOF(.got) == 0, "
ERROR(mimxrt500-rt): .got section detected in the input object files
Dynamic relocations are not supported. If you are linking to C code compiled using
the 'cc' crate then modify your build script to compile the C code _without_
the -fPIC flag. See the documentation of the `cc::Build.pic` method for details.");
/* Do not exceed this mark in the error messages above                                    | */
