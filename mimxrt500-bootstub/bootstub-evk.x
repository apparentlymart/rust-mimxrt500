MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
}

SECTIONS
{
  .mimxrt500_bootstub ORIGIN(FLASH) :
  {
    FILL(0x00000000)
    . = ORIGIN(FLASH) + 0x0400;
    __mimxrt500_bootstub_start = .;
    KEEP(*(.mimxrt500_bootstub.exceptions));
    KEEP(*(.mimxrt500_bootstub .mimxrt500_bootstub.*));
    __mimxrt500_bootstub_end = .;
    . = ORIGIN(FLASH) + 0x1000;
    FILL(0xffffffff)
  } > FLASH
}
