#![allow(non_camel_case_types)]
use crate::{impl_bitfield, register::bitfield::BitField, register::register::Register};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PGABypasMode {
    ENABLE = 0b0,
    BYPASS = 0b1,
}
impl_bitfield!(PGABypasMode, 7, 7, Register::MODE2);

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PGAGain {
    GAIN_1 = 0b000,
    GAIN_2 = 0b001,
    GAIN_4 = 0b010,
    GAIN_8 = 0b011,
    GAIN_16 = 0b100,
    GAIN_32 = 0b101,
}
impl_bitfield!(PGAGain, 4, 6, Register::MODE2);

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DataRate {
    RATE_2_5 = 0b0000,
    RATE_5 = 0b0001,
    RATE_10 = 0b0010,
    RATE_16_6 = 0b0011,
    RATE_20 = 0b0100,
    RATE_50 = 0b0101,
    RATE_60 = 0b0110,
    RATE_100 = 0b0111,
    RATE_400 = 0b1000,
    RATE_1200 = 0b1001,
    RATE_2400 = 0b1010,
    RATE_4800 = 0b1011,
    RATE_7200 = 0b1100,
    RATE_14400 = 0b1101,
    RATE_19200 = 0b1110,
    RATE_38400 = 0b1111,
}
impl_bitfield!(DataRate, 0, 3, Register::MODE2);
