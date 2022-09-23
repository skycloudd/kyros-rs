#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kyros_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kyros_rs::println;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    kyros_rs::init();

    #[cfg(test)]
    test_main();

    println!("hello world");

    kyros_rs::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    kyros_rs::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kyros_rs::test_panic_handler(info)
}
