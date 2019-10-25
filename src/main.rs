#![no_std]
#![no_main]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]

mod gpio;

use core::panic::PanicInfo;


fn main() {
    loop {
        let s = "I'm a main";
        for letter in s.as_bytes() {
            let _l = *letter;
        }
    }
}


//***Vectors***//
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    //The reset vector is just entering the main function
    main();
    loop {
    }
}

#[repr(C)]
pub union Vector{
    vector_handler: unsafe extern "C" fn() -> !,
    unused_vector: u32
}

impl Vector{
    pub const fn handler(handler: unsafe extern "C" fn() -> !) -> Vector{
        Vector{vector_handler: handler}
    }
    pub const fn unused() -> Vector{
        Vector{unused_vector: 0}
    }
}

//***Vector table***//
#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: [Vector; 14] = [
    Vector::handler(Reset),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
    Vector::unused(),
];

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}