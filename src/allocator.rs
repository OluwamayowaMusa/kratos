use core::{
    alloc::GlobalAlloc,
    ptr::{addr_of, addr_of_mut},
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::{multiboot::MultibootInfo, println};

#[global_allocator]
pub static ALLOC: Allocator = Allocator::new();

#[repr(C, packed)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FreeSegment {
    pub size: usize,
    pub next_segment: *mut FreeSegment,
}

impl FreeSegment {
    fn get_start(&self) -> *mut u8 {
        unsafe { (self as *const FreeSegment).add(1) as *mut u8 }
    }

    fn get_end(&self) -> *mut u8 {
        unsafe { self.get_start().add(self.size) }
    }

    fn update_size(&mut self, ptr: *mut u8) {
        unsafe {
            self.size = ptr
                .offset_from(self.get_start())
                .try_into()
                .expect("Expected a valid usize");
        }
    }
}

#[repr(C, packed)]
struct UsedSegment {
    size: usize,
    padding: [u8; 4],
}

impl UsedSegment {
    fn get_start(&self) -> *mut u8 {
        unsafe { (self as *const UsedSegment).add(1) as *mut u8 }
    }

    fn update_size(&mut self, ptr: *mut u8) {
        unsafe {
            self.size = ptr
                .offset_from(self.get_start())
                .try_into()
                .expect("Expected a vaild usize");
        }
    }
}

pub struct Allocator {
    pub first_free: AtomicPtr<FreeSegment>,
}

impl Allocator {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Allocator {
        Allocator {
            first_free: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn init(&self, info: &MultibootInfo) {
        assert_eq!(
            core::mem::size_of::<UsedSegment>(),
            core::mem::size_of::<FreeSegment>()
        );
        let kernel_start_addr = addr_of!(crate::libc::KERNEL_START) as u64;
        let kernel_end_addr = addr_of!(crate::libc::KERNEL_END) as u64;
        let big_block = info
            .get_memmory_map()
            .iter()
            .find(
                |entry| entry.addr == kernel_start_addr - 1024 * 1024, // Kernel start at 2M due to UEFI
            )
            .expect("Failed to find big block of ram");

        // Kernel is in big block
        let reserved_memory = kernel_end_addr - big_block.addr;
        let segment_size =
            (big_block.len - reserved_memory) as usize - core::mem::size_of::<FreeSegment>();

        let segment = addr_of_mut!(crate::libc::KERNEL_END) as *mut FreeSegment;
        *segment = FreeSegment {
            size: segment_size,
            next_segment: core::ptr::null_mut(),
        };
        self.first_free.store(segment, Ordering::Relaxed);
        println!("Allocator Initialized");
    }
}

unsafe fn get_header_ptr(segment: &FreeSegment, layout: &core::alloc::Layout) -> Option<*mut u8> {
    let segment_start = segment.get_start();
    let segment_end = segment.get_end();
    let mut ptr = segment_end.sub(layout.size());
    ptr = ptr.sub((ptr as usize) % layout.align());
    ptr = ptr.sub(core::mem::size_of::<UsedSegment>());

    if ptr < segment_start {
        println!("Segment size too small");
        return None;
    }

    Some(ptr)
}

unsafe fn get_header_ptr_from_allocated(ptr: *mut u8) -> *mut UsedSegment {
    ptr.sub(core::mem::size_of::<UsedSegment>()) as *mut UsedSegment
}

unsafe fn merge_if_adjacent(a: *mut FreeSegment, b: *mut FreeSegment) {
    if (a as *mut u8).add((*a).size + core::mem::size_of::<FreeSegment>()) == b as *mut u8 {
        (*a).size = (*a).size + core::mem::size_of::<FreeSegment>() + (*b).size;
        (*a).next_segment = (*b).next_segment
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn print_free_segment(list: *mut FreeSegment) {
    let mut iterator = list;
    while !iterator.is_null() {
        println!(
            "{:?} {:?} {:?}",
            iterator,
            (*iterator).get_start(),
            *iterator
        );
        iterator = (*iterator).next_segment;
    }
}

unsafe fn convert_used_to_free_segment(
    list_head: *mut FreeSegment,
    used_segment: *mut UsedSegment,
) {
    let size = (*used_segment).size;
    let new_free_segment = used_segment as *mut FreeSegment;
    (*new_free_segment).size = size;
    (*new_free_segment).next_segment = core::ptr::null_mut();

    insert_segment_into_list(list_head, new_free_segment)
}

unsafe fn insert_segment_into_list(list_head: *mut FreeSegment, new_segment: *mut FreeSegment) {
    let mut iterator = list_head;

    while !iterator.is_null() {
        assert!(iterator < new_segment);

        let should_insert =
            (*iterator).next_segment.is_null() || (*iterator).next_segment > new_segment;
        if should_insert {
            let next = (*iterator).next_segment;
            (*iterator).next_segment = new_segment;
            (*new_segment).next_segment = next;

            merge_if_adjacent(new_segment, (*new_segment).next_segment);
            merge_if_adjacent(iterator, new_segment);

            return;
        }

        iterator = (*iterator).next_segment;
    }

    panic!("Failed to insert segment into list");
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut free_block_it = self.first_free.load(Ordering::Relaxed);
        while !free_block_it.is_null() {
            let header_ptr = get_header_ptr(&*free_block_it, &layout);
            let header_ptr = match header_ptr {
                Some(v) => v,
                None => {
                    free_block_it = (*free_block_it).next_segment;
                    continue;
                }
            };

            // Store the segment end before updating the size
            let segment_end = (*free_block_it).get_end();

            (*free_block_it).update_size(header_ptr);

            let header_ptr = header_ptr as *mut UsedSegment;
            (*header_ptr).update_size(segment_end);

            return (*header_ptr).get_start();
        }
        panic!("Failed to alloc");
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let header_ptr = get_header_ptr_from_allocated(ptr);

        convert_used_to_free_segment(self.first_free.load(Ordering::Relaxed), header_ptr)
    }
}
