// Project:     rust-os
// Author:      Gunnar Westerling
// Date:        10.04.2023
// License:     MIT
// File:        main.rs
// Description: This is the main file of the kernel.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    loop {}
}