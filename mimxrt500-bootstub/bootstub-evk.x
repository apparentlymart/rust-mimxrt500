/* This linker script is for a standalone boot stub suitable for the
   MIMXRT595-EVK board. The boot stub occupies the first 64KiB of
   the flash, and then a normal cortex-m-rt-style application can
   be separately written starting at offset 0x10000.

   This linker script is not suitable for a bootstub embedded directly
   into an application, but could be adapted to achieve that by using
   the same section names.
*/

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
}

SECTIONS
{
  .mimxrt500_bootstub ORIGIN(FLASH) :
  {
    __mimxrt500_bootstub_reserved_start = .;
    FILL(0xffffffff)

    /* Flash Control Block: this must be present for the boot ROM to
       consider this flash as a valid boot candidate. */
    . = ORIGIN(FLASH) + 0x0400;
    __mimxrt500_bootstub_fcb_start = .;
    KEEP(*(.mimxrt500_bootstub.fcb));
    __mimxrt500_bootstub_fcb_end = .;

    /* Initial vector table: this is used only during early boot
       and contains some image metadata nestled in some fields that
       are normally reserved on ARMv8-M. */
    . = ORIGIN(FLASH) + 0x1000;
    __mimxrt500_bootstub_vectors_start = .;
    KEEP(*(.mimxrt500_bootstub.exceptions));
    __mimxrt500_bootstub_vectors_end = .;
    __mimxrt500_bootstub_start = .;
    KEEP(*(.mimxrt500_bootstub .mimxrt500_bootstub.*));
    __mimxrt500_bootstub_end = .;

    __mimxrt500_bootstub_reserved_end = ORIGIN(FLASH) + 0x10000;

    /* This address (64KiB into flash) is where the main application
       vector table should be placed. */
    __mimxrt500_bootstub_app_start = ORIGIN(FLASH) + 0x10000;
  } > FLASH

  /DISCARD/ : {
    *(.init);
    *(.fini);
  }
}
