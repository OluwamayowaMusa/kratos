use core::fmt::Write;
use thiserror_no_std::Error;

use crate::io::port_manager::{Port, PortManager};
use crate::println;

const BASE_ADDR: u16 = 0x3f8; // COM1
const ISA_DEBUG_EXIT_PORT_NUM: u16 = 0xf4; // EXIT PORT

#[derive(Debug, Error)]
pub enum SerialInitError {
    #[error("Data port reserved")]
    DataReserved,
    #[error("Enable port reserved")]
    EnableReserved,
    #[error("Interrupt Id port reserved")]
    InterruptIdReserved,
    #[error("Line Control port reserved")]
    LineControlReserved,
    #[error("Modem Control port reserved")]
    ModemControlReserved,
    #[error("Line Staus port reserved")]
    LineStatusReserved,
    #[error("Modem Staus port reserved")]
    ModemStatusReserved,
    #[error("Scratch port reserved")]
    ScratchReserved,
    #[error("Loopback test failed")]
    Loopback,
    #[error("Port is reserved")]
    Exit,
}

pub struct Serial {
    data: Port,
    enable_interrupt: Port,
    interrupt_id_fifo_control: Port,
    line_control: Port,
    modem_control: Port,
    line_status: Port,
    _modem_status: Port,
    _scratch: Port,
    pub exit: Port,
}

impl Serial {
    pub fn new(port_manager: &mut PortManager) -> Result<Serial, SerialInitError> {
        use SerialInitError::*;

        let data = port_manager.request_port(BASE_ADDR).ok_or(DataReserved)?;
        let enable_interrupt = port_manager
            .request_port(BASE_ADDR + 1)
            .ok_or(EnableReserved)?;
        let interrupt_id_fifo_control = port_manager
            .request_port(BASE_ADDR + 2)
            .ok_or(InterruptIdReserved)?;
        let line_control = port_manager
            .request_port(BASE_ADDR + 3)
            .ok_or(LineControlReserved)?;
        let modem_control = port_manager
            .request_port(BASE_ADDR + 4)
            .ok_or(ModemControlReserved)?;
        let line_status = port_manager
            .request_port(BASE_ADDR + 5)
            .ok_or(LineStatusReserved)?;
        let modem_status = port_manager
            .request_port(BASE_ADDR + 6)
            .ok_or(ModemStatusReserved)?;
        let scratch = port_manager
            .request_port(BASE_ADDR + 7)
            .ok_or(ScratchReserved)?;
        let exit = port_manager
            .request_port(ISA_DEBUG_EXIT_PORT_NUM)
            .ok_or(Exit)?;

        Ok(Serial {
            data,
            enable_interrupt,
            interrupt_id_fifo_control,
            line_control,
            modem_control,
            line_status,
            _modem_status: modem_status,
            _scratch: scratch,
            exit,
        })
    }

    pub fn init(&self) -> Result<(), SerialInitError> {
        self.enable_interrupt.writeb(0x00); // Disable all interrupts
        self.line_control.writeb(0x80); // Enable DLAB (set baud rate divisor)
        self.data.writeb(0x03); // Set divisor to 3 (lo byte) 38400 baud
        self.enable_interrupt.writeb(0x00); //     (hi byte)
        self.line_control.writeb(0x03); // 8 bits, no parity, one stop bit
        self.interrupt_id_fifo_control.writeb(0xC7); // Enable FIFO, clear them, with 14-byte threshold
        self.modem_control.writeb(0x0B); // IRQs enabled, RTS/DSR set
        self.modem_control.writeb(0x1E); // Set in loopback mode, test the serial chip
        self.data.writeb(0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if self.data.readb() != 0xAE {
            return Err(SerialInitError::Loopback);
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        self.modem_control.writeb(0x0F);

        println!("Serial driver initialized");

        Ok(())
    }

    pub fn write(&self, text: &str) {
        for &character in text.as_bytes() {
            self.write_serial(character)
        }
    }

    fn is_transmit_full(&self) -> bool {
        self.line_status.readb() & 0x20 == 0
    }

    fn write_serial(&self, character: u8) {
        while self.is_transmit_full() {}

        self.data.writeb(character);
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);

        Ok(())
    }
}
