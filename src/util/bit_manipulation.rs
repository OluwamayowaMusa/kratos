use core::ops;

pub fn get_bits<T>(value: T, position: T, length: T) -> T
where
    T: ops::Shr<Output = T>
        + ops::Div<Output = T>
        + ops::Shl<Output = T>
        + ops::Sub<Output = T>
        + ops::BitAnd<Output = T>
        + core::fmt::Display
        + core::marker::Copy,
{
    #[allow(clippy::eq_op)]
    let index = length / length;
    let mask = (index << length) - index;

    (value >> position) & mask
}

pub fn get_bit<T>(value: T, position: T) -> T
where
    T: ops::Shr<Output = T>
        + ops::Div<Output = T>
        + ops::Shl<Output = T>
        + ops::Sub<Output = T>
        + ops::BitAnd<Output = T>
        + core::fmt::Display
        + core::marker::Copy,
{
    #[allow(clippy::eq_op)]
    get_bits(value, position, value / value)
}

pub fn set_bits<T>(input: &mut T, position: T, length: T, value: T)
where
    T: ops::Div<Output = T>
        + ops::Shl<Output = T>
        + ops::Sub<Output = T>
        + ops::BitAndAssign
        + ops::BitOrAssign
        + core::fmt::Display
        + core::marker::Copy
        + ops::Not<Output = T>
        + ops::BitAnd<Output = T>
        + ops::ShlAssign,
{
    #[allow(clippy::eq_op)]
    let index = length / length;
    let mut mask = (index << length) - index;
    mask <<= position;

    *input &= !mask;
    *input |= (value << position) & mask
}

pub fn set_bit<T>(input: &mut T, position: T, enable: bool)
where
    T: ops::Div<Output = T>
        + ops::Shl<Output = T>
        + ops::Sub<Output = T>
        + ops::BitAndAssign
        + ops::BitOrAssign
        + core::fmt::Display
        + core::marker::Copy
        + ops::Not<Output = T>
        + ops::BitAnd<Output = T>
        + ops::ShlAssign,
{
    #[allow(clippy::eq_op)]
    let index = *input / *input;
    #[allow(clippy::eq_op)]
    let value = {
        if enable {
            *input / *input
        } else {
            *input - *input
        }
    };

    set_bits(input, position, index, value)
}
