MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(Reset);
/*EXTERN(RESET_VECTOR);*/
SECTIONS
{
  . = 0x8000000;
  .vector_table ORIGIN(FLASH) :
  {
    /*First, the stack pointer*/
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    /*Then, the address of the reset vector*/
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text :
  {

  } > FLASH

  .rodata :
  {

  } > RAM
   .ARM.exidx :
  {

  } > RAM

}