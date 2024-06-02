// Disable standard libary
#![no_std]
// Disable rust entry point
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Libray
use kratos::multiboot::{print_mmap_sections, MultibootInfo};
use kratos::vga::{Terminal, VgaColor};

// Include the boot.s which includes the _start function which is the entry point of the program
// Rust's ASM block does not seem to default to at&t syntax. Use `options(att_syntax)`
global_asm!(include_str!("boot.s"), options(att_syntax));

// Defines the behavior of panic
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[allow(clippy::empty_loop, clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kernel_main(_magic: u32, info: *const MultibootInfo) -> ! {
    let mut terminal = Terminal::new(VgaColor::LightGrey, VgaColor::Black);
    terminal.init().expect("Terminal not initialized");
    print_mmap_sections(&mut terminal, info);

    loop {}
}
