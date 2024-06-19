pub mod port_manager; // Contains Port related functions
pub mod serial; // Contains Serial related functions
pub mod vga; // Contains VGA related functions

#[macro_export]
macro_rules! print {
    ( $ ( $arg:tt )* ) => {
        #[allow(clippy::macro_metavars_in_unsafe)]
        unsafe {
            use core::fmt::Write;
            write!($crate::io::vga::TERMINAL.borrow_mut(), $($arg)*).expect("Not Written to VGA");

            let serial_writer = core::ptr::addr_of_mut!($crate::io::serial::SERIAL);
            write!(&mut *serial_writer, $($arg)*).expect("Not Written to Serial")
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
