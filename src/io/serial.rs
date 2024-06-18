use crate::println;
use core::{arch::asm, fmt::Write};

const PORT: u16 = 0x3f8; // COM1

pub static mut SERIAL: Serial = Serial {};

#[derive(Debug)]
pub struct SerialInitError;

pub struct Serial {}

impl Serial {
    pub unsafe fn init(&self) -> Result<(), SerialInitError> {
        outb(PORT + 1, 0x00); // Disable all interrupts
        outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        outb(PORT + 1, 0x00); //                  (hi byte)
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
        outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
        outb(PORT + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(PORT + 0) != 0xAE {
            return Err(SerialInitError);
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(PORT + 4, 0x0F);

        println!("Serial driver initialized");

        Ok(())
    }

    pub fn write(&self, text: &str) {
        unsafe {
            for &character in text.as_bytes() {
                Serial::write_serial(character)
            }
        }
    }

    unsafe fn is_transmit_empty() -> u8 {
        inb(PORT + 5) & 0x20
    }

    unsafe fn write_serial(character: u8) {
        while Serial::is_transmit_empty() == 0 {}

        outb(PORT, character);
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);

        Ok(())
    }
}

// Store val in reg `al`
// Store addr in reg `dx`
// Write the value of `al` to `dx`
unsafe fn outb(addr: u16, val: u8) {
    asm!(r#"
        out %al, %dx
        "#,
    in("dx") addr,
    in("al") val,
    options(att_syntax)
    );
}

// Store addr in `dx`
// Set `al` to out register
// Read the value of dx into al
// Store the value of `al` into ret
unsafe fn inb(addr: u16) -> u8 {
    let mut ret;
    asm!(r#"
        in %dx, %al
        "#,
    in("dx") addr,
    out("al") ret,
    options(att_syntax)
    );

    ret
}

pub unsafe fn exit(code: u8) {
    const SHUTDOWN_PORT: u16 = 0xf4;
    outb(SHUTDOWN_PORT, code);
}
