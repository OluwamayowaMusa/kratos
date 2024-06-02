// Disable standard library
#![no_std]
// Convert raw pointer to &str
#![feature(str_from_raw_parts)]

pub mod libc; // Contains C related functions
pub mod multiboot;
pub mod vga; // Contains VGA related functions // Contains Multiboot related functions
