use core::cell::RefCell;
use core::arch::asm;

use crate::{io::port_manager::PortManager, println, util::bit_manipulation::{set_bit, set_bits}};

static INTERRUPT_TABLE: InterruptTable = InterruptTable::new();

struct InterruptTable {
    inner: RefCell<[GateDescriptor; 256]>
}

impl InterruptTable {
    const fn new() -> InterruptTable {
        InterruptTable {
            inner: RefCell::new([GateDescriptor(0); 256]),
        }
    }
}

unsafe impl Sync for InterruptTable {}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct GateDescriptor(u64);

impl GateDescriptor {
    fn new(params: GateDescriptorParams) -> GateDescriptor {
        let mut descriptor = 1u64;

        set_bits(&mut descriptor, 0, 16, params.offset as u64);
        set_bits(&mut descriptor, 48, 16, (params.offset >> 16) as u64);
        set_bits(&mut descriptor, 16, 16, params.segment_selector as u64);
        set_bits(&mut descriptor, 40, 4, params.gate_type as u64);
        set_bits(&mut descriptor, 45, 2, params.dpl as u64);
        set_bit(&mut descriptor, 47, params.p);

        GateDescriptor(descriptor)
    }
}

struct GateDescriptorParams {
    offset: u32,
    segment_selector: u16, 
    gate_type: u8,
    dpl: u8,
    p: bool,
}


pub fn init(port_manager: &mut PortManager) {
    let master_pic_data = port_manager.request_port(0x21).expect("Failed to get master data port");
    let slave_pic_data = port_manager.request_port(0xA1).expect("Failed to get slave data port");
    
    // Disable External Interrupts
    master_pic_data.writeb(0xFF);
    slave_pic_data.writeb(0xFF);

    let general_fault_descriptor = GateDescriptor::new(GateDescriptorParams {
        offset: general_fault_handler as u32,
        segment_selector: 0x08,
        gate_type: 0b1111,
        dpl: 0,
        p: true,

    });

    let mut table = INTERRUPT_TABLE.inner.borrow_mut();
    table[13] = general_fault_descriptor; 
    let size = table.len() * core::mem::size_of::<GateDescriptor>() - 1;
    let table_ptr = table.as_ptr();

    let idt = Idt {
        size: size as u16,
        base: table_ptr as u32,

    };
    
    println!("Initial IDT: {:?}", read_idtr());

    unsafe {
        asm!(r#"
            lidt ({idt})
            sti
            int $80
            "#,
            idt = in(reg) &idt,
            options(att_syntax),
        );
    }

    println!("Updated IDT {:?}", read_idtr());
}

fn read_idtr() -> Idt {
    let mut ret = core::mem::MaybeUninit::uninit();
    unsafe {
        asm!(r#"
            sidt ({ret})
            "#,
            ret = in(reg) ret.as_mut_ptr(),
            options(att_syntax, nostack, preserves_flags),
        );

        ret.assume_init()
    }
}

#[repr(C, packed)]
#[derive(Debug)]
struct Idt {
    size: u16,
    base: u32,
}

extern "x86-interrupt" fn general_fault_handler() {
    //println!("Helloo");
}

