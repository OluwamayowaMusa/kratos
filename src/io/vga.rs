use core::fmt::Write; // Write Formatted arguments

// VGA text mode color constants
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub static mut TERMINAL: Terminal = Terminal::new(VgaColor::LightGrey, VgaColor::Black);

#[allow(dead_code)]
pub enum VgaColor {
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

pub struct Terminal {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: *mut u16,
}

unsafe impl Sync for Terminal {}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_text(s.as_bytes());

        Ok(())
    }
}

impl Terminal {
    pub const fn new(fore_ground_color: VgaColor, back_ground_color: VgaColor) -> Terminal {
        let terminal_row = 0;
        let terminal_column = 0;
        let terminal_color = Terminal::set_color(fore_ground_color, back_ground_color);
        let terminal_buffer = 0xB8000 as *mut u16;

        Terminal {
            terminal_row,
            terminal_column,
            terminal_color,
            terminal_buffer,
        }
    }

    pub fn init(&mut self) -> core::fmt::Result {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *self.terminal_buffer.add(index) = self.set_screen_character(b' ');
                }
            }
        }

        Ok(())
    }

    pub fn write_text(&mut self, text: &[u8]) {
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

    fn write_character(&mut self, character: u8) {
        let index = self.terminal_row * VGA_WIDTH + self.terminal_column;
        unsafe {
            *self.terminal_buffer.add(index) = self.set_screen_character(character);
        }

        self.set_cursor()
    }

    const fn set_color(fore_ground_color: VgaColor, back_ground_color: VgaColor) -> u8 {
        fore_ground_color as u8 | (back_ground_color as u8) << 4
    }

    fn set_screen_character(&self, character: u8) -> u16 {
        character as u16 | (self.terminal_color as u16) << 8
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
}
