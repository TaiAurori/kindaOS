#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
// #![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
extern crate alloc;
pub mod interrupts;
pub mod vga_buffer;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod shell;
use crate::vga_buffer::{ColorCode, Color};

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn npanic() -> ! {
    color!(ColorCode::new(Color::LightRed, Color::Red));
    print!("KERNEL PANIC!");
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
