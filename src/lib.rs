use command::Command;
use register::bitfield::BitField;
use register::inpmux::{PositiveInpmux, NegativeInpmux};
use register::register::Register;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

mod register;
mod command;

pub const CHECK_BYTE: u8 = 0x9B;
pub const CRC_BYTE: u64 = 0b100000111;
pub const DUMMY: u8 = 0x00;

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

pub struct ADS1263<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
{
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> ADS1263<SPI, CS>
where
    SPI: Transfer<u8, Error = E>,
    CS: OutputPin<Error = E>,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }

    /// Send a command to the device
    pub fn send_command(&mut self, command: Command) -> Result<(), E> {
        self.cs.set_low()?;
        self.spi.transfer(&mut [command as u8])?;
        self.cs.set_high()?;
        Ok(())
    }

    /// Read the contents of a register as a u8
    pub fn read_register(&mut self, register: Register) -> Result<u8, E> {
        let mut buf = [register as u8 | Command::RREG as u8, 0, 0];
        self.cs.set_low()?;
        let out = self.spi.transfer(&mut buf)?;
        self.cs.set_high()?;
        Ok(out[2])
    }

    /// Write the contents of a register as a u8
    pub fn write_register(&mut self, register: Register, val: u8) -> Result<(), E> {
        let mut buf = [register as u8 | Command::WREG as u8, 0, val];
        self.cs.set_low()?;
        self.spi.transfer(&mut buf)?;
        self.cs.set_high()?;
        Ok(())
    }

    /// Utility function to set specific bits in a register
    /// Reads the register sets the bits then writes it back to device
    /// Offset and register is handled by the BitField trait
    pub fn write_bitfield<T>(&mut self, bitfield: T) -> Result<(), E> 
    where T: BitField
    {
        let mut register = self.read_register(bitfield.register())?;
        register &= !bitfield.mask();
        register |= (bitfield.into()) << bitfield.start();
        self.write_register(bitfield.register(), register)
    }

    /// Read voltage difference between two pins using ADC1
    /// Does not compute CRC or checksum
    pub fn read_adc1(&mut self, positive_pin: PositiveInpmux, negative_pin: NegativeInpmux) -> Result<u32, E> {
        let interface = self.read_register(Register::INTERFACE)?;
        let status_byte_enable = (interface & 0b100) != 0;
        let checksum_byte_enable = interface & 0b11;

        self.write_bitfield(positive_pin)?;
        self.write_bitfield(negative_pin)?;

        let mut buf = vec![Command::RDATA1 as u8, 0, 0, 0, 0];
        let mut data_index = 1;
        if status_byte_enable {
            buf.push(0);
            data_index += 1;
        }
        if checksum_byte_enable != 0b00 {
            buf.push(0);
        }

        let out = self.spi.transfer(&mut buf)?;

        Ok(as_u32_be(&[
            out[data_index],
            out[data_index + 1],
            out[data_index + 2],
            out[data_index + 3],
        ]))
    }
}
