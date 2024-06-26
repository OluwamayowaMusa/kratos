use crate::println;

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

impl MultibootInfo {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_memory_map(&self) -> &[MultibootMmapEntry] {
        let number_of_memory_segments =
            self.mmap_length as usize / core::mem::size_of::<MultibootMmapEntry>();
        core::slice::from_raw_parts(
            self.mmap_addr as *const MultibootMmapEntry,
            number_of_memory_segments,
        )
    }
}

// Low field contains important data
#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootMmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub type_: u32,
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn print_mmap_sections(info: *const MultibootInfo) {
    let boot_loader_name = core::str::from_raw_parts((*info).boot_loader_name, 5);
    println!("Boot Loader name: {}", boot_loader_name);

    let mut total_memmory = 0;
    println!("Available memory segments");
    let mmap_length = (*info).mmap_length;
    println!("mmap_length: {}", mmap_length);

    for memory in (*info).get_memory_map() {
        let size = memory.size;
        let len = memory.len;
        let addr = memory.addr;
        let type_ = memory.type_;
        total_memmory += len;

        println!("Size: {size}, len: {len}, addr: {addr:#0X}, type : {type_}");
    }
    println!(
        "Total Memory: {}MB",
        (total_memmory as f32) / 1024.0 / 1024.0
    );
    println!("End of memory segments.");
}
