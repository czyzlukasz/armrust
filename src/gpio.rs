struct Register{
    GPIO_CRL: u32,
    GPIO_CRH: u32,
    GPIO_IDR: u32,
    GPIO_ODR: u32,
    GPIO_BSRR: u32,
    GPIO_BRR: u32,
    GPIO_LCKR: u32
}

impl Register{
    fn new() -> Register{
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

pub static PORTA: Register = Register::new();
pub static PORTB: Register = Register::new();
pub static PORTC: Register = Register::new();
pub static PORTD: Register = Register::new();
pub static PORTE: Register = Register::new();
pub static PORTF: Register = Register::new();
pub static PORTG: Register = Register::new();
