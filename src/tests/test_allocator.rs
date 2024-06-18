use alloc::{boxed::Box, vec::Vec};
use kratos::allocator::FreeSegment;

// Test macros
use crate::create_test;
use crate::tests::TestCase;



unsafe fn  capture_alloc_state() -> [FreeSegment; 100] {
    let mut list = [FreeSegment {
        size: 0,
        next_segment: core::ptr::null_mut(),

    }; 100];
    let mut index = 0;
    let mut first_free_block = kratos::allocator::ALLOC.first_free.load(core::sync::atomic::Ordering::Relaxed);

    while !first_free_block.is_null() {
        list[index] = *first_free_block;

        index += 1;
        first_free_block = (*first_free_block).next_segment;
    }

    list
}

create_test!(test_simple_alloc, {
    unsafe {
        let initial_state = capture_alloc_state();
        let temp = Box::new(4);
        
        // Different states due to heap allocation
        assert_ne!(initial_state, capture_alloc_state());

        let alloc_state = capture_alloc_state();
        let num_diff = initial_state.iter().zip(alloc_state.iter()).filter(|(a, b)| a != b).count();
        
        // Difference should be 1
        assert_eq!(num_diff, 1);


        let (before, after) = initial_state.iter().zip(alloc_state.iter()).find(|(a, b)| a != b).expect("Could not find any");

        // before size > after size
        assert!(before.size > after.size);

        drop(temp);
        
        // States should be the same after drop
        assert_eq!(initial_state, capture_alloc_state());
        Ok(())
    }
});


create_test!(test_nested_vector_alloc, {
    unsafe {
        let initial_state = capture_alloc_state();
        {
            let mut v = Vec::new();
            const NUM_ALLOCATIONS: usize = 10;
            
            for i in 0..NUM_ALLOCATIONS {
                let mut v2 = Vec::new();
                for j in 0..i {
                    v2.push(j);
                }
                v.push(v2);
            }

            for i in (0..NUM_ALLOCATIONS - 1).filter(|x| (x % 2) == 0).rev() {
                let len = v.len() - 1;
                v.swap(len, i);
                v.pop();
            }
                
            {
                for i in 0..NUM_ALLOCATIONS {
                    let mut v2 = Vec::new();
                    for j in 0..i {
                        v2.push(j);
                    }
                    v.push(v2);
                }

            }
        
            for elem in v {
                for (i, item) in elem.into_iter().enumerate() {
                    assert_eq!(i, item);
                }
            }
        }
        
        // State should be the same after block
        assert_eq!(initial_state, capture_alloc_state());
        Ok(())
    }
});
