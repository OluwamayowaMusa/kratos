pub mod port_manager; // Contains Port related functions
pub mod serial; // Contains Serial related functions
pub mod vga; // Contains VGA related functions

use core::cell::RefCell;

use port_manager::PortManager;
use serial::Serial;
use vga::{Terminal, VgaColor};

pub static DISPLAY: Display = Display {
    inner: RefCell::new(DisplayInner {
        vga: None,
        serial: None,
    }),
};

#[macro_export]
macro_rules! print {
    ( $ ( $arg:tt )* ) => {
        #[allow(clippy::macro_metavars_in_unsafe)]
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write;
            let mut display = $crate::io::DISPLAY.borrow_mut();
            if let Some(vga) = &mut display.vga {
                write!(vga, $($arg)*).expect("Not Written to VGA");
            }

            if let Some(serial) = &mut display.serial {
                write!(serial, $($arg)*).expect("Not Written to Serial")
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    ( $ ( $arg:tt )* ) => {
        $crate::print!($($arg)*);
        $crate::print!("\n");
    };
}

pub struct Display {
    inner: RefCell<DisplayInner>,
}

unsafe impl Sync for Display {}

impl core::ops::Deref for Display {
    type Target = RefCell<DisplayInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct DisplayInner {
    pub vga: Option<Terminal>,
    pub serial: Option<Serial>,
}

pub fn init_display(port_manager: &mut PortManager) {
    let mut vga = Terminal::new(VgaColor::LightGrey, VgaColor::Black);
    vga.init().expect("Unable to initialize VGA display");

    let serial = Serial::new(port_manager).expect("Unable to create Serial");
    serial.init().expect("Unable to initialize Serial Display");

    let mut display = DISPLAY.borrow_mut();
    display.vga = Some(vga);
    display.serial = Some(serial);
}

pub fn exit(code: u8) {
    let display = DISPLAY.borrow();
    if let Some(serial) = &display.serial {
        serial.exit.writeb(code);
    }
}
