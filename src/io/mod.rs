pub mod vga; // Contains VGA related functions 
pub mod serial; // Contains Serial related functions



#[macro_export]
macro_rules! print {
    ( $ ( $arg:tt )* ) => {
        #[allow(clippy::macro_metavars_in_unsafe)]
        unsafe {
            use core::fmt::Write;
            let vga_writer = core::ptr::addr_of_mut!($crate::io::vga::TERMINAL);
            write!(&mut *vga_writer, $($arg)*).expect("Not Written to VGA");

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
