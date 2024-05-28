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

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, character: u8, size: usize) {
    for index in 0..size {
        *ptr.add(index) = character;
    }
}

#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut terminal = Terminal::new();
    terminal.write_text(b"Musa is a GOAT\n");

    loop {}
}

fn vga_entry_color(fore_ground_color: VgaColor, back_ground_color: VgaColor) -> u8 {
    fore_ground_color as u8 | (back_ground_color as u8) << 4
}

fn vga_entry(character: u8, color: u8) -> u16 {
    character as u16 | (color as u16) << 8
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
                unsafe {
                    *terminal_buffer.add(index) = vga_entry(b' ', terminal_color)
                }
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
            self.write_character(character);
        }
    }

}
