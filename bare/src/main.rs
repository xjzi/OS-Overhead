#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use work;
use tsc;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Keep this function name in the binary
#[no_mangle]
// The linker by default calls _start()
pub extern "C" fn _start() -> ! {
    println!("Starting hashing.");

    let start = tsc::start();

    work::work();

    let stop = tsc::stop();

    let duration = stop - start;

    println!("Hashing completed.");
    println!("Started at: {}", start);
    println!("Stopped at: {}", stop);
    println!("Duration: {}", duration);

    loop {}
}