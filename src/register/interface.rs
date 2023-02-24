use crate::{register::bitfield::BitField, impl_bitfield, register::register::Register};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum SerialInterfaceTimeout {
    DISABLED = 0,
    ENABLED = 1,
}
impl_bitfield!(SerialInterfaceTimeout, 3, 3, Register::INTERFACE);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum StatusByte {
    DISABLED = 0,
    ENABLED = 1,
}
impl_bitfield!(StatusByte, 2, 2, Register::INTERFACE);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ChecksumByte {
    DISABLED = 0b0,
    CHECKSUM = 0b1,
    CRC = 0b10,
}
impl_bitfield!(ChecksumByte, 0, 1, Register::INTERFACE);
