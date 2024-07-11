use alloc::vec::Vec;
use core::arch::asm;
use core::cell::RefCell;

// Library
use crate::println;
use crate::util::bit_manipulation::{get_bits, set_bit, set_bits};

static GDT_ENTRIES: GdtTable = GdtTable::new();

struct GdtTable {
    inner: RefCell<Vec<GdtSegemt>>,
}

impl core::ops::Deref for GdtTable {
    type Target = RefCell<Vec<GdtSegemt>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GdtTable {
    const fn new() -> GdtTable {
        GdtTable {
            inner: RefCell::new(Vec::new()),
        }
    }
}

unsafe impl Sync for GdtTable {}

#[repr(C, packed)]
#[derive(Clone)]
pub struct GdtSegemt(u64);

impl GdtSegemt {
    pub fn new(base: u32, limit: u32, access_byte: u8, flags: u8) -> GdtSegemt {
        let mut descriptor = 1u64;
        set_bits(&mut descriptor, 0, 16, limit as u64);
        set_bits(&mut descriptor, 48, 4, (limit >> 16) as u64);

        set_bits(&mut descriptor, 16, 24, base as u64);
        set_bits(&mut descriptor, 56, 8, (base >> 24) as u64);

        set_bits(&mut descriptor, 40, 8, access_byte as u64);

        set_bits(&mut descriptor, 52, 4, flags as u64);

        GdtSegemt(descriptor)
    }

    pub fn base(&self) -> u32 {
        let data = self.0;
        let mut base = get_bits(data, 16, 24);
        let upper = get_bits(data, 56, 8);
        base |= upper << 24;
        base as u32
    }

    pub fn limit(&self) -> u32 {
        let data = self.0;
        let mut limit = get_bits(data, 0, 16);
        let upper = get_bits(data, 48, 4);
        limit |= upper << 16;
        limit as u32
    }

    pub fn access(&self) -> u8 {
        let data = self.0;
        get_bits(data, 40, 8) as u8
    }

    pub fn flags(&self) -> u8 {
        let data = self.0;
        get_bits(data, 52, 4) as u8
    }
}

#[repr(C, packed)]
struct Gdt {
    limit: u16,
    base: u32,
}

fn read_gdtr() -> Gdt {
    let mut ret = core::mem::MaybeUninit::uninit();
    unsafe {
        asm!(r#"
            sgdt [{ret}]
            "#,
            ret = in(reg) ret.as_mut_ptr(),
            options(nostack, preserves_flags),
        );

        ret.assume_init()
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn print_gdtr() {
    let gdt = read_gdtr();
    let liimt = gdt.limit + 1;
    let base = gdt.base as *const GdtSegemt;
    println!("Base: {:?}, Limit: {}", base, liimt);

    for index in 0..(liimt / 8) {
        println!("Segment [{}]", index);
        let segment = base.add(index.into());
        let base = (*segment).base();
        let limit = (*segment).limit();
        let access = (*segment).access();
        let flags = (*segment).flags();

        println!(
            "Base: {:#x}, Limit: {:#x}, Access: {:#x}, Flags: {:#x}",
            base, limit, access, flags
        );
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn init() {
    let mut entries = GDT_ENTRIES.borrow_mut();
    *entries = get_gdt_vals().to_vec();

    let entry_addres: *const GdtSegemt = entries.as_ptr();
    let limit = entries.len() * core::mem::size_of::<GdtSegemt>() - 1;

    let gdt = Gdt {
        limit: limit as u16,
        base: entry_addres as u32,
    };

    asm!(r#"
        lgdt [{}]
        "#,
        in(reg) &gdt
    );
}

fn get_gdt_vals() -> [GdtSegemt; 3] {
    let access_byte = generate_access_byte(AccessByteParams {
        p: true,
        dpl: 0,
        s: true,
        e: true,
        dc: false,
        rw: false,
        a: true,
    });
    let code = GdtSegemt::new(0, 0xFFFFF, access_byte, 0b1100);

    let access_byte = generate_access_byte(AccessByteParams {
        p: true,
        dpl: 0,
        s: true,
        e: false,
        dc: false,
        rw: true,
        a: true,
    });
    let data = GdtSegemt::new(0, 0xFFFFF, access_byte, 0b1100);

    [GdtSegemt(0), code, data]
}

struct AccessByteParams {
    p: bool,
    dpl: u8,
    s: bool,
    e: bool,
    dc: bool,
    rw: bool,
    a: bool,
}

fn generate_access_byte(params: AccessByteParams) -> u8 {
    let mut access_byte = 1u8;
    set_bit(&mut access_byte, 7, params.p);
    set_bits(&mut access_byte, 5, 2, params.dpl);
    set_bit(&mut access_byte, 4, params.s);
    set_bit(&mut access_byte, 3, params.e);
    set_bit(&mut access_byte, 2, params.dc);
    set_bit(&mut access_byte, 1, params.rw);
    set_bit(&mut access_byte, 0, params.a);

    access_byte
}
