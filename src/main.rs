#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kyros_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kyros_rs::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    print, println,
    task::{executor::Executor, keyboard, Task},
    vga_buffer::BUFFER_SIZE,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kyros_rs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut output = String::from("[welcome to kyros]");

    for _ in 0..(BUFFER_SIZE.width - output.len()) / 2 {
        output.insert(0, ' ');
    }

    println!("{}", output);
    println!();

    println!();
    print!("> ");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));

    executor.run();
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
