// Disable standard libary
#![no_std]
// Disable rust entry point
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Include the boot.s which includes the _start function which is the entry point of the program
// Rust's ASM block does not seem to default to at&t syntax. Use `options(att_syntax)`
global_asm!(include_str!("boot.s"), options(att_syntax));

// Defines the behavior of panic
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

// Multiboot information
#[repr(C, packed)]
pub struct MultibootInfo {
    // Multiboot info version number
    flags: u32,

    // Available memory from BIOS
    mem_lower: u32,
    mem_upper: u32,

    // Root partition
    boot_device: u32,

    // Kernel command line
    cmdline: u32,

    // Boot-Module list
    mods_count: u32,
    mods_addr: u32,

    // Store the syms data of Multiboot info
    dummy: u128,

    // memory Mapping Buffer
    mmap_length: u32,
    mmap_addr: u32,

    // Drive info bvffer
    drives_length: u32,
    drives_addr: u32,

    // ROM  configuration table
    config_table: u32,

    // Boot Loader name
    boot_loader_name: *const u8,
}

#[repr(C, packed)]
struct MultibootMmapEntry {
    size: u32,
    addr: u64,
    len: u64,
    type_: u32,
}

// VGA text mode color constants
#[allow(dead_code)]
enum VgaColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magneta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagneta,
    LightBrown,
    White,
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const BASE_DIGIT: usize = 48;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, character: u8, size: usize) {
    for index in 0..size {
        *ptr.add(index) = character;
    }
}

#[allow(clippy::empty_loop, clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn kernel_main(_magic: u32, info: *const MultibootInfo) -> ! {
    let mut terminal = Terminal::new();

    for index in 0..(*info).mmap_length {
        let mmap_entry = ((*info).mmap_addr
            + core::mem::size_of::<MultibootMmapEntry>() as u32 * index)
            as *const MultibootMmapEntry;

        terminal.write_text(b"size: ");
        terminal.write_numbers((*mmap_entry).size as usize);
        terminal.write_text(b" len: ");
        terminal.write_numbers((*mmap_entry).len as usize);
        terminal.write_text(b" addr: ");
        terminal.write_numbers((*mmap_entry).addr as usize);
        terminal.write_text(b" type: ");
        terminal.write_numbers((*mmap_entry).type_ as usize);
        terminal.write_text(b"\n");
    }

    loop {}
}

fn vga_entry_color(fore_ground_color: VgaColor, back_ground_color: VgaColor) -> u8 {
    fore_ground_color as u8 | (back_ground_color as u8) << 4
}

fn vga_entry(character: u8, color: u8) -> u16 {
    character as u16 | (color as u16) << 8
}

// Get the least power of 10 greater than the number
fn get_number_divisor(number: usize) -> usize {
    let mut divisor = 10;
    while number >= divisor {
        divisor *= 10;
    }

    divisor / 10
}

struct Terminal {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: *mut u16,
}

impl Terminal {
    fn new() -> Terminal {
        let terminal_row = 0;
        let terminal_column = 0;
        let terminal_color = vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
        let terminal_buffer = 0xB8000 as *mut u16;
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe { *terminal_buffer.add(index) = vga_entry(b' ', terminal_color) }
            }
        }

        Terminal {
            terminal_row,
            terminal_column,
            terminal_color,
            terminal_buffer,
        }
    }

    fn set_cursor(&mut self) {
        self.terminal_column += 1;
        if self.terminal_column == VGA_WIDTH {
            self.terminal_column = 0;
            self.terminal_row += 1;
            if self.terminal_row == VGA_HEIGHT {
                self.terminal_row = 0;
            }
        }
    }

    fn write_character(&mut self, character: u8) {
        let index = self.terminal_row * VGA_WIDTH + self.terminal_column;
        unsafe {
            *self.terminal_buffer.add(index) = vga_entry(character, self.terminal_color);
        }

        self.set_cursor()
    }

    fn write_text(&mut self, text: &[u8]) {
        for &character in text {
            match character {
                b'\n' => self.handle_new_line(),
                _ => self.write_character(character),
            }
        }
    }

    fn handle_new_line(&mut self) {
        self.terminal_column = 0;
        self.terminal_row += 1;
        if self.terminal_row == VGA_HEIGHT {
            self.terminal_row = 0;
        }
    }

    // Postive Numbers Presently
    fn write_numbers(&mut self, number: usize) {
        match number {
            0..10 => self.write_character((BASE_DIGIT + number) as u8),
            _ => {
                let mut divisor = get_number_divisor(number);

                while divisor > 0 {
                    let digit = (number / divisor) % 10;
                    self.write_numbers(digit);
                    divisor /= 10;
                }
            }
        }
    }
}
