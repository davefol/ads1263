#![allow(non_camel_case_types)]
use crate::{register::bitfield::BitField, impl_bitfield, register::register::Register};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConversionDelay {
    DELAY_0 = 0b0000,
    DELAY_8700 = 0b0001,
    DELAY_17000 = 0b0010,
    DELAY_35000 = 0b0011,
    DELAY_69000 = 0b0100,
    DELAY_139000 = 0b0101,
    DELAY_278000 = 0b0110,
    DELAY_555000 = 0b0111,
    DELAY_1_1 = 0b1000,
    DELAY_2_2 = 0b1001,
    DELAY_4_4 = 0b1010,
    DELAY_8_8 = 0b1011,
}
impl_bitfield!(ConversionDelay, 0, 3, Register::MODE0);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum InputChop {
    DISABLED = 0,
    ENABLED = 1,
}
impl_bitfield!(InputChop, 4, 4, Register::MODE0);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum IDACRotation {
    DISABLED = 0,
    ENABLED = 1,
}
impl_bitfield!(IDACRotation, 5, 5, Register::MODE0);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ADCConversionRunMode {
    CONTINUOUS = 0,
    PULSE = 1,
}

impl_bitfield!(ADCConversionRunMode, 6, 6, Register::MODE0);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum RefMuxPolarity {
    NORMAL = 0,
    REVERSED = 1,
}
impl_bitfield!(RefMuxPolarity, 7, 7, Register::MODE0);
