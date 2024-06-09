// Disable standard libary
#![no_std]
// Disable rust entry point
#![no_main]
#![feature(panic_info_message)]
extern crate alloc;
use alloc::vec;
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::addr_of;

// Libray
use kratos::allocator::Allocator;
use kratos::libc::{get_esp, KERNEL_END, KERNEL_START};
use kratos::multiboot::{print_mmap_sections, MultibootInfo};
use kratos::println;
use kratos::vga::TERMINAL;

#[global_allocator]
static ALLOC: Allocator = Allocator::new();

// Include the boot.s which includes the _start function which is the entry point of the program
// Rust's ASM block does not seem to default to at&t syntax. Use `options(att_syntax)`
global_asm!(include_str!("boot.s"), options(att_syntax));

// Defines the behavior of panic
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(args) = panic_info.message() {
        println!("{}", args);
    } else {
        println!("Panicked in else");
    }
    loop {}
}

#[allow(clippy::empty_loop, clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kernel_main(_magic: u32, info: *const MultibootInfo) -> ! {
    TERMINAL.init().expect("Terminal not initialized");

    ALLOC.init(&*info);

    println!("Stack Pointer: {:#x}", get_esp());
    println!(
        "Kernel start: {:?} Kernel End: {:?}",
        addr_of!(KERNEL_START),
        addr_of!(KERNEL_END)
    );

    print_mmap_sections(info);

    let initial_state = *ALLOC.first_free.load(core::sync::atomic::Ordering::Relaxed);
    let list = vec![1, 2, 3, 4];
    println!("List: {:?}", list);
    drop(list);
    let after_state = *ALLOC.first_free.load(core::sync::atomic::Ordering::Relaxed);
    assert_eq!(initial_state, after_state);

    loop {}
}
