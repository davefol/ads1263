use crate::{impl_bitfield, register::bitfield::BitField, register::register::Register};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ReferencePositiveInput {
    INT = 0b000,
    AIN0 = 0b001,
    AIN2 = 0b010,
    AIN4 = 0b011,
    VDD = 0b100,
}
impl_bitfield!(ReferencePositiveInput, 3, 5, Register::REFMUX);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ReferenceNegativeInput {
    INT = 0b000,
    AIN1 = 0b001,
    AIN3 = 0b010,
    AIN5 = 0b011,
    VSS = 0b100,
}
impl_bitfield!(ReferenceNegativeInput, 0, 2, Register::REFMUX);
