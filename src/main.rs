#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;
use kinda_os::vga_buffer::{Color, ColorCode};
use kinda_os::{println, color, memory, allocator, shell};
use kinda_os::task::{executor::Executor};
use kinda_os::memory::BootInfoFrameAllocator;
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Booting kindaOS...");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    kinda_os::init();
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let mut executor = Executor::new(); // new
    shell::init(&mut executor);
    executor.run();
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    color!(ColorCode::new(Color::Red, Color::Black));
    println!("\n{}", _info);
    kinda_os::npanic()
}