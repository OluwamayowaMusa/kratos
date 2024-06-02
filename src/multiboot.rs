use crate::vga::Terminal;
use core::fmt::Write;

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

// Low field contains important data
#[repr(C, packed)]
struct MultibootMmapEntry {
    size: u32,
    addr_low: u32,
    addr_high: u32,
    len_low: u32,
    len_high: u32,
    type_: u32,
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn print_mmap_sections(terminal: &mut Terminal, info: *const MultibootInfo) {
    let boot_loader_name = (*info).boot_loader_name;
    let name_slice = core::str::from_raw_parts(boot_loader_name, 5);
    writeln!(terminal, "Boot Loader name: {}", name_slice).expect("Not Written");

    let mmap_length = (*info).mmap_length as usize;
    let base_addr = (*info).mmap_addr as *const MultibootMmapEntry;
    writeln!(terminal, "Available memory segments").expect("Not Written");
    writeln!(terminal, "mmap_length: {}", mmap_length).expect("Not Written");

    for index in 0..mmap_length {
        let memory = base_addr.add(index);
        let size = (*memory).size;
        let len = (*memory).len_low;
        let addr = (*memory).addr_low;
        let type_ = (*memory).type_;

        if size == 0 {
            writeln!(terminal, "End of memory segments").expect("Not Written");
            break;
        }
        writeln!(
            terminal,
            "Size: {size}, len: {len}, addr: {addr}, type : {type_}"
        )
        .expect("Not Written");
    }
}
