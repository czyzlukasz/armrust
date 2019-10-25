#![allow(non_snake_case)]
#![allow(dead_code)]

pub enum Pin{
    Pin0 = 1,
    Pin1 = 2,
    Pin2 = 4,
    Pin3 = 8,
    Pin4 = 16,
    Pin5 = 32,
    Pin6 = 64,
    Pin7 = 128,
    Pin8 = 256,
    Pin9 = 512,
    Pin10 = 1024,
    Pin11 = 2048,
    Pin12 = 4096,
    Pin13 = 8192,
    Pin14 = 16384,
    Pin15 = 32768
}

#[repr(C)]
pub struct Register{
    GPIO_CRL: u32,
    GPIO_CRH: u32,
    GPIO_IDR: u32,
    GPIO_ODR: u32,
    GPIO_BSRR: u32,
    GPIO_BRR: u32,
    GPIO_LCKR: u32
}

impl Register{
    const fn new() -> Register{
        Register{
            // Reset values for the GPIO registers
            GPIO_CRL: 0b01000100_01000100_01000100_01000100,
            GPIO_CRH: 0b01000100_01000100_01000100_01000100,
            GPIO_IDR: 0,
            GPIO_ODR: 0,
            GPIO_BSRR: 0,
            GPIO_BRR: 0,
            GPIO_LCKR: 0
        }
    }

    fn set_bit(&mut self, pin: Pin){
        self.GPIO_BSRR = pin as u32;
    }

    fn reset_bit(&mut self, pin: Pin){
        self.GPIO_BSRR = (pin as u32) << 16;
    }
}

#[link_section = ".gpio_a"]
#[no_mangle]
pub static PORTA: Register = Register::new();
#[link_section = ".gpio_b"]
#[no_mangle]
pub static PORTB: Register = Register::new();
#[link_section = ".gpio_c"]
#[no_mangle]
pub static PORTC: Register = Register::new();
#[link_section = ".gpio_d"]
#[no_mangle]
pub static PORTD: Register = Register::new();
#[link_section = ".gpio_e"]
#[no_mangle]
pub static PORTE: Register = Register::new();
#[link_section = ".gpio_f"]
#[no_mangle]
pub static PORTF: Register = Register::new();
#[link_section = ".gpio_g"]
#[no_mangle]
pub static PORTG: Register = Register::new();
