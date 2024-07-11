// Disable standard library
#![no_std]
// Convert raw pointer to &str
#![feature(str_from_raw_parts)]

extern crate alloc;
pub mod allocator; // Contains Memory allocator functions
pub mod gdt; // Contains Global descriptor Table related functions
pub mod io; // Contains IO related functions;
pub mod libc; // Contains C related functions
pub mod multiboot; // Contains Multiboot specification related functions
pub mod util; // Contains utilities and helper functions
