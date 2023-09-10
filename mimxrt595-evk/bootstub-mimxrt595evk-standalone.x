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
  FCB : ORIGIN = 0x08000400, LENGTH = 512
  BOOTSTUB : ORIGIN = 0x08001000, LENGTH = 61440
}

SECTIONS
{
  /* Flash Control Block: this must be present for the boot ROM to
     consider this flash as a valid boot candidate. */
  .mimxrt500_bootstub.fcb ORIGIN(FCB) :
  {
    FILL(0xffffffff)
    __mimxrt500_bootstub_fcb_start = .;
    KEEP(*(.mimxrt500_bootstub.fcb));
    __mimxrt500_bootstub_fcb_end = .;
  } > FCB

  /* Initial vector table and boot code: this is used only during early
     boot and contains some image metadata nestled in some fields that
     are normally reserved on ARMv8-M. */
  .mimxrt500_bootstub.text ORIGIN(BOOTSTUB) :
  {
    FILL(0xffffffff)
    __mimxrt500_bootstub_image_start = .;
    __mimxrt500_bootstub_vectors_start = .;
    KEEP(*(.mimxrt500_bootstub.exceptions));
    __mimxrt500_bootstub_vectors_end = .;
    __mimxrt500_bootstub_start = .;
    KEEP(*(.mimxrt500_bootstub .mimxrt500_bootstub.*));
    __mimxrt500_bootstub_end = .;
  } > BOOTSTUB

  /* This address (64KiB into flash) is where the main application
      vector table should be placed. */
  __mimxrt500_bootstub_app_start = 0x08010000;

  /DISCARD/ : {
    *(.init);
    *(.fini);
    *(.text .text.*);
    *(.ARM.*);
    *(.comment);
    *(.debug_frame);
  }
}
