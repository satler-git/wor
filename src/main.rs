#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

/// Call on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Test Massage");
    println!("Hello world");

    loop {}
}
