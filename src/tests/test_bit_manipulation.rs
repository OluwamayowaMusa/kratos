use kratos::util::bit_manipulation::{get_bit, get_bits, set_bit, set_bits};

use crate::create_test;
use crate::tests::TestCase;

create_test!(test_bit_manipulation, {
    assert_eq!(get_bits(0b111001, 2, 4), 14);
    assert_eq!(get_bit(0b111001, 1), 0);
    let mut input = 0b11001;
    set_bits(&mut input, 1, 3, 0b010);
    assert_eq!(input, 0b10101);
    set_bit(&mut input, 3, true);
    assert_eq!(input, 0b11101);
    set_bit(&mut input, 0, false);
    assert_eq!(input, 0b11100);
    Ok(())
});
