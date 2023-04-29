#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_support::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod test_support;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Hello again");
    println!(", some numbers: {} {}", 42, 1.337);

    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::test_support::{exit_qemu, QemuExitCode};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(2, 2);
}
