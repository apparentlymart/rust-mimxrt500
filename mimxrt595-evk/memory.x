MEMORY
{
  FCB (r) : ORIGIN = 0x08000400, LENGTH = 512
  BOOTSTUB (rx) : ORIGIN = 0x08001000, LENGTH = 61440
  FLASH (rx) : ORIGIN = 0x08010000, LENGTH = 64M - 64K
  REBOOTRAM (w) : ORIGIN = 0x00000000, LENGTH = 32K
  RAM (wx) : ORIGIN = 0x00008000, LENGTH = 5088K
}

_stack_start = ORIGIN(REBOOTRAM) + 4096;

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
}
