MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(Reset);
EXTERN(VECTOR_TABLE);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    /*Address of the reset vector*/
    .vector_table;
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