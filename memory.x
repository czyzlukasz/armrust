MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(Reset);
EXTERN(MSP);
EXTERN(RESET_VECTOR);

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .msp_start ORIGIN(FLASH) :
  {
    /*Initial value of MSP*/
    .msp_start;
  } > FLASH

  .vector_table.reset_vector :
  {
    /*Address of the reset vector*/
    .vector_table.reset_vector;
  } > FLASH

  .text :
  {

  } > FLASH

  .rodata :
  {

  } > FLASH

   .ARM.exidx :
  {

  } > RAM
}