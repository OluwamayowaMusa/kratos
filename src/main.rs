// Disable standard libary
#![no_std]
// Disable rust entry point
#![no_main]
// Panic
#![feature(panic_info_message)]
// Test
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(tests::test_runner)]

extern crate alloc;
use alloc::vec;
use core::arch::{global_asm, asm};
use core::panic::PanicInfo;
use core::ptr::addr_of;

// External Crate
use hashbrown::HashMap;

// Libray
use kratos::{gdt, io};
use kratos::libc::{get_esp, KERNEL_END, KERNEL_START};
use kratos::multiboot::{print_mmap_sections, MultibootInfo};
use kratos::println;

// Contains Test
#[cfg(test)]
mod tests;

// Static Variables
use kratos::allocator::ALLOC;

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

    io::exit(1);

    loop {}
}

#[allow(clippy::missing_safety_doc)]
fn read_cs_reg() -> *const u16 {
    let mut ret;
    unsafe {
        asm!(r#"
            mov %cs, {ret}
            "#,
            ret = out(reg) ret,
            options(att_syntax)
        );
    }
    ret
}

fn read_gdtr() -> u64 {
    let mut ret: u64 = 0;
    unsafe {
        asm!(r#"
            sgdt [{ret}]
            "#,
            ret = in(reg) &mut ret,
            options(nostack, preserves_flags),
        );
    }

    ret
}

fn print_gdt_segment(segment: u16) {
    println!("Segment: {:#x} {:#b}", segment, segment);
}

#[allow(clippy::empty_loop, clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kernel_main(_magic: u32, info: *const MultibootInfo) -> ! {
    ALLOC.init(&*info);

    let mut port_manager = io::port_manager::PortManager::new();
    io::init_display(&mut port_manager);
    println!("Display Initialized");

    #[cfg(test)]
    {
        test_main();
        io::exit(0);
    }

    println!("Stack Pointer: {:#x}", get_esp());
    println!(
        "Kernel start: {:?} Kernel End: {:?}",
        addr_of!(KERNEL_START),
        addr_of!(KERNEL_END)
    );

    print_mmap_sections(info);

    {
        println!("A vector: {:?}", vec![1, 2, 3, 4]);
        let mut a_map = HashMap::new();
        a_map.insert("age", 4);
        println!("A map: {:?}", a_map);
    }

    print_gdt_segment(*read_cs_reg());
    gdt::init();
    println!("gdt: {:#x}", read_gdtr());

    let rtc = io::rtc::Rtc::new(&mut port_manager).expect("Failed to create RTC");
    let mut date = rtc.read();
    println!("Current date: {:?}", date);
    date.hours -= 1;
    rtc.write(&date);

    let date = rtc.read();
    println!("Current date modified: {:?}", date);

    loop {}
}
