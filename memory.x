MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

ENTRY(Reset);
EXTERN(VECTOR_TABLE);

EXTERN(PORTA);
EXTERN(PORTB);
EXTERN(PORTC);
EXTERN(PORTD);
EXTERN(PORTE);
EXTERN(PORTF);
EXTERN(PORTG);

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

/*   Memory mapped registers   */
  .gpio_a 0x40010800 : {.gpio_a} > FLASH
  .gpio_b 0x40010C00 : {.gpio_b} > FLASH
  .gpio_c 0x40011000 : {.gpio_c} > FLASH
  .gpio_d 0x40011400 : {.gpio_d} > FLASH
  .gpio_e 0x40011800 : {.gpio_e} > FLASH
  .gpio_f 0x40011C00 : {.gpio_f} > FLASH
  .gpio_g 0x40012000 : {.gpio_g} > FLASH

   .ARM.exidx :
  {

  } > RAM
}