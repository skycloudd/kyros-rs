#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kyros_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kyros_rs::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kyros_rs::init();

    #[cfg(test)]
    test_main();

    println!("type something:",);

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
