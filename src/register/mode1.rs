#![allow(non_camel_case_types)]
use crate::{impl_bitfield, register::bitfield::BitField, register::register::Register};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DigitalFilter {
    SINC1 = 0b000,
    SINC2 = 0b001,
    SINC3 = 0b010,
    SINC4 = 0b011,
    FIR = 0b100,
}
impl_bitfield!(DigitalFilter, 5, 7, Register::MODE1);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum SensorBiasADC {
    ADC1 = 0,
    ADC2 = 1,
}
impl_bitfield!(SensorBiasADC, 4, 4, Register::MODE1);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum SensorBiasPolarity {
    PULL_UP = 0,
    PULL_DOWN = 1,
}
impl_bitfield!(SensorBiasPolarity, 3, 3, Register::MODE1);

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum SensorBiasMagnitude {
    NONE = 0b000,
    MA05 = 0b001,
    MA2 = 0b010,
    MA10 = 0b011,
    MA50 = 0b100,
    MA200 = 0b101,
    MO10 = 0b110,
}
impl_bitfield!(SensorBiasMagnitude, 0, 2, Register::MODE1);
