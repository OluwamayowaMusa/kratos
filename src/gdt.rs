use core::arch::asm;

use crate::println;

static GDT_ENTRIES: [u64; 3] = get_gdt_vals();

pub fn init() {
    let limit = core::mem::size_of_val(&GDT_ENTRIES) - 1;
    let gdt = (&GDT_ENTRIES as *const u64 as u64) << 16 | limit as u64;
    println!("{:?}", &GDT_ENTRIES as *const u64);
    unsafe {
        asm!(r#"
            lgdt [{}]
            "#,
            in(reg) &gdt
        );
    }
}

pub const fn get_gdt_vals() -> [u64; 3] {
    let access_byte = AccessByteBulider::new()
        .set_p(true)
        .set_dpl(0)
        .set_s(true)
        .set_e(true)
        .set_dc(false)
        .set_rw(false)
        .set_a(true)
        .build();
    let mut flags:u16 = 0b1100 << 12;
    flags |= access_byte as u16;
    let code = create_descriptor(0, 0xFFFFF, flags);

    let access_byte = AccessByteBulider::new()
        .set_p(true)
        .set_dpl(0)
        .set_s(true)
        .set_e(false)
        .set_dc(false)
        .set_rw(true)
        .set_a(true)
        .build();
    let mut flags:u16 = 0b1100 << 12;
    flags |= access_byte as u16;
    let data = create_descriptor(0, 0xFFFFF, flags);

    [0u64, code, data]
}

pub struct AccessByteBulider {
    access_byte: u8,
}

impl AccessByteBulider {
    pub const fn new() -> Self {
        AccessByteBulider {
            access_byte: 0,
        }
    }

    pub const fn set_p(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 7);
        self
    }

    pub const fn set_dpl(mut self, val: u8) -> Self {
        self.access_byte = set_bit(self.access_byte, val >> 1 & 0x1 == 1, 6);
        self.access_byte = set_bit(self.access_byte, val & 0x1 == 1, 5);
        self
    }

    pub const fn set_s(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 4);
        self
    }

    pub const fn set_e(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 3);
        self
    }

    pub const fn set_dc(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 2);
        self
    }

    pub const fn set_rw(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 1);
        self
    }

    pub const fn set_a(mut self, enable: bool) -> Self {
        self.access_byte = set_bit(self.access_byte, enable, 0);
        self
    }

    pub const fn build(self) -> u8 {
        self.access_byte
    }

}

pub const fn set_bit(mut input: u8, enable: bool, bit: u8) -> u8 {
    if enable {
        input |= 1 << bit
    } else {
        input &= !(1 << bit)
    }

    input
}

pub const fn create_descriptor(base: u32, limit: u32, flag: u16) -> u64 {

    // Create the high 32 bit segment
    let mut descriptor: u64  =  (limit & 0x000F0000) as u64; // set limit bits 19:16
    descriptor |= (flag <<  8) as u64 & 0x00F0FF00; // set type, p, dpl, s, g, d/b, l and avl fields
    descriptor |= (base >> 16) as u64 & 0x000000FF; // set base bits 23:16
    descriptor |= (base & 0xFF000000) as u64; // set base bits 31:24
 
    // Shift by 32 to allow for low part of segment
    descriptor <<= 32;
 
    // Create the low 32 bit segment
    descriptor |= (base  << 16) as u64; // set base bits 15:0
    descriptor |= (limit  & 0x0000FFFF) as u64; // set limit bits 15:0
 
    descriptor
}

pub fn parse_base(segment: u64) -> u64 {
    let base_mask_lower = 0x0000_00ff_ffff_0000_u64;
    let base_mask_upper = 0xff00_0000_0000_0000_u64;
    let mut base = (segment & base_mask_lower) >> 16;
    base |= (segment & base_mask_upper) >> 32;
    base
}


pub fn parse_limit(segment: u64) -> u64 {
    let limit_mask_lower = 0x0000_0000_0000_ffff_u64;
    let limit_mask_upper = 0x000f_0000_0000_0000_u64;
    let mut limit = segment & limit_mask_lower;
    limit |= (segment & limit_mask_upper) >> 32;
    limit
}
