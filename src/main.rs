#![no_std]
#![no_main]

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

//***Vector table***//
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;



#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}