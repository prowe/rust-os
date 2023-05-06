#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_support::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod test_support;
mod vga_buffer;
mod interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    #[cfg(test)]
    test_main();

    print_at!(2, 2, "┌───────────────┐");
    print_at!(3, 2, "│ Hello World ! │");
    print_at!(4, 2, "└───────────────┘");

    loop {}
}

fn init() {
    interrupts::init_idt();
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

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(2, 2);
    }
}
