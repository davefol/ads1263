use crate::{register::bitfield::BitField, impl_bitfield, register::register::Register};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum PositiveInpmux {
    AIN0 = 0b0000,
    AIN1 = 0b0001,
    AIN2 = 0b0010,
    AIN3 = 0b0011,
    AIN4 = 0b0100,
    AIN5 = 0b0101,
    AIN6 = 0b0110,
    AIN7 = 0b0111,
    AIN8 = 0b1000,
    AIN9 = 0b1001,
    AINCOM = 0b1010,
    TEMP = 0b1011,
    ANALOG = 0b1100,
    DIGITAL = 0b1101,
    TDAC = 0b1110,
    FLOAT = 0b1111,
}
impl_bitfield!(PositiveInpmux, 4, 7, Register::INPMUX);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum NegativeInpmux {
    AIN0 = 0b0000,
    AIN1 = 0b0001,
    AIN2 = 0b0010,
    AIN3 = 0b0011,
    AIN4 = 0b0100,
    AIN5 = 0b0101,
    AIN6 = 0b0110,
    AIN7 = 0b0111,
    AIN8 = 0b1000,
    AIN9 = 0b1001,
    AINCOM = 0b1010,
    TEMP = 0b1011,
    ANALOG = 0b1100,
    DIGITAL = 0b1101,
    TDAC = 0b1110,
    FLOAT = 0b1111,
}
impl_bitfield!(NegativeInpmux, 0, 3, Register::INPMUX);
