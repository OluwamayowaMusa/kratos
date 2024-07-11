use kratos::gdt::{create_descriptor, parse_base, parse_limit, set_bit, AccessByteBulider};

use crate::create_test;
use crate::tests::TestCase;

create_test!(test_limit_extraction, {
        assert_eq!(parse_limit(0x0000000000000000), 0x00000);
        assert_eq!(parse_limit(0x00CF9A000000FFFF), 0xFFFFF);
        Ok(())
    }
);

create_test!(test_base_extraction, {
        assert_eq!(parse_base(0x0000000000000000), 0x0_u64);
        assert_eq!(parse_base(0x120D00345678BEEF), 0x12345678);
        Ok(())
});

create_test!(test_descriptor_creation, {
        let base = 0x12345678;
        let limit = 0xABCDE;
        let descriptor = create_descriptor(base, limit, 0);
        assert_eq!(parse_limit(descriptor), limit as u64);
        assert_eq!(parse_base(descriptor), base as u64);
        Ok(())
});

create_test!(test_clear_bit, {
        let input = 0xff;
        assert_eq!(set_bit(input, false, 4), 0xef);
        Ok(())
});

create_test!(test_set_bit, {
        let input = 0x00;
        assert_eq!(set_bit(input, true, 3), 0x8);
        Ok(())
});

create_test!(test_access_byte_code_segment, {
        let access_byte = AccessByteBulider::new()
                .set_p(true)
                .set_dpl(0)
                .set_s(true)
                .set_e(true)
                .set_dc(false)
                .set_rw(false)
                .set_a(true)
                .build();
        assert_eq!(access_byte, 0b10011001);
        Ok(())
});
