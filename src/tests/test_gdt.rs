use kratos::gdt::GdtSegemt;

use crate::create_test;
use crate::tests::TestCase;

create_test!(test_gdt_functions, {
    let segment = GdtSegemt::new(0x12345678, 0xDBEEF, 0xFF, 0b1100);
    assert_eq!(segment.base(), 0x12345678);
    assert_eq!(segment.limit(), 0xDBEEF);
    assert_eq!(segment.access(), 0xFF);
    assert_eq!(segment.flags(), 0b1100);
    Ok(())
});
