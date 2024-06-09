extern "C" {
    pub static KERNEL_START: u32;
    pub static mut KERNEL_END: u32; // Mutable due to the initialization of the the free segement
    pub fn get_esp() -> u32;
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, character: u8, size: usize) {
    for index in 0..size {
        *ptr.add(index) = character;
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, size: usize) {
    for index in 0..size {
        *dest.add(index) = *src.add(index);
    }
}

#[allow(clippy::missing_safety_doc)]
#[allow(clippy::comparison_chain)]
#[no_mangle]
pub unsafe extern "C" fn memcmp(string1: *const u8, string2: *const u8, size: usize) -> i8 {
    for index in 0..size {
        if *string1.add(index) < *string2.add(index) {
            return -1;
        } else if *string1.add(index) > *string2.add(index) {
            return 1;
        }
    }

    0
}
