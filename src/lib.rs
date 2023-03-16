use command::Command;
use embedded_hal::{
    blocking::{delay::DelayMs, spi::Transfer},
    digital::v2::{InputPin, OutputPin},
};
use register::bitfield::BitField;
use register::inpmux::{NegativeInpmux, PositiveInpmux};
use register::register::Register;

pub mod command;
pub mod register;

pub const CHECK_BYTE: u8 = 0x9B;
pub const CRC_BYTE: u64 = 0b100000111;
pub const DUMMY: u8 = 0x00;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Error<S, P> {
    SpiError(S),
    PinError(P),
    DataReadyTimeoutError,
    DRDYPinNotConfigured,
    InvalidResponseFromDevice,
}

pub enum Checksum {
    Checksum(u8),
    CRC(u8),
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

pub struct ADS1263<SPI, CS, DRDY, Delay>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
    DRDY: InputPin,
    Delay: DelayMs<u32>
{
    spi: SPI,
    cs: CS,
    drdy: Option<DRDY>,
    delay: Delay
}

impl<SPI, CS, DRDY, Delay, S, P> ADS1263<SPI, CS, DRDY, Delay>
where
    SPI: Transfer<u8, Error = S>,
    CS: OutputPin<Error = P>,
    DRDY: InputPin<Error = P>,
    Delay: DelayMs<u32>
{
    pub fn new(spi: SPI, cs: CS, drdy: Option<DRDY>, delay: Delay) -> Self {
        Self { spi, cs, drdy, delay }
    }

    /// Send a command to the device
    pub fn send_command(&mut self, command: Command) -> Result<(), Error<S, P>> {
        self.cs.set_low().map_err(Error::PinError)?;
        self.spi
            .transfer(&mut [command as u8])
            .map_err(Error::SpiError)?;
        self.cs.set_high().map_err(Error::PinError)?;
        Ok(())
    }

    /// Read the contents of a register as a u8
    pub fn read_register(&mut self, register: Register) -> Result<u8, Error<S, P>> {
        let mut buf = [register as u8 | Command::RREG as u8, 0, 0];
        self.cs.set_low().map_err(Error::PinError)?;
        let out = self.spi.transfer(&mut buf).map_err(Error::SpiError)?;
        self.cs.set_high().map_err(Error::PinError)?;
        Ok(out[2])
    }

    /// Write the contents of a register as a u8
    pub fn write_register(&mut self, register: Register, val: u8) -> Result<(), Error<S, P>> {
        let mut buf = [register as u8 | Command::WREG as u8, 0, val];
        self.cs.set_low().map_err(Error::PinError)?;
        self.spi.transfer(&mut buf).map_err(Error::SpiError)?;
        self.cs.set_high().map_err(Error::PinError)?;
        Ok(())
    }

    /// Utility function to set specific bits in a register
    /// Reads the register sets the bits then writes it back to device
    /// Offset and register is handled by the BitField trait
    pub fn write_bitfield<T>(&mut self, bitfield: T) -> Result<(), Error<S, P>>
    where
        T: BitField,
    {
        let mut register = self.read_register(bitfield.register())?;
        register &= !bitfield.mask();
        register |= (bitfield.into()) << bitfield.start();
        self.write_register(bitfield.register(), register)
    }

    /// Read voltage difference between two pins using ADC1
    /// Does not compute CRC or checksum
    pub fn read_adc1(
        &mut self,
        positive_pin: PositiveInpmux,
        negative_pin: NegativeInpmux,
    ) -> Result<u32, Error<S, P>> {
        self.write_bitfield(positive_pin)?;
        self.write_bitfield(negative_pin)?;

        if self.drdy.is_some() {
            self.wait_for_pin_data_ready()?;
            let (_, data, _) = self.read_data1()?;
            return Ok(data);
        } else {
            self.write_bitfield(register::interface::StatusByte::ENABLED)?;
            loop {
                let (status, data, _) = self.read_data1()?;
                if let Some(status) = status {
                    if (status & 0b01000000) != 0 {
                        return Ok(data);
                    }
                }
            }
        }
    }

    fn read_data1(&mut self) -> Result<(Option<u8>, u32, Option<Checksum>), Error<S, P>> {
        // read_command, optional status byte, u32 data, optional checksum
        let mut buf = [Command::RDATA1 as u8, 0, 0, 0, 0, 0, 0];

        // data index gets shifted if status_byte is enabled
        let interface = self.read_register(Register::INTERFACE)?;
        let status_byte_enable = (interface & 0b100) != 0;
        let checksum_byte_enable = interface & 0b11;

        let data_index = match status_byte_enable {
            false => 1,
            true => 2,
        };

        self.cs.set_low().map_err(Error::PinError)?;
        let out = self.spi.transfer(&mut buf).map_err(Error::SpiError)?;
        self.cs.set_high().map_err(Error::PinError)?;

        let status_byte = match status_byte_enable {
            false => None,
            true => Some(out[1]),
        };

        let data = as_u32_be(&[
            out[data_index],
            out[data_index + 1],
            out[data_index + 2],
            out[data_index + 3],
        ]);

        let checksum_byte = match checksum_byte_enable {
            0 => Ok(None),
            0b01 => Ok(Some(Checksum::Checksum(out[data_index + 4]))),
            0b10 => Ok(Some(Checksum::CRC(out[data_index + 4]))),
            _ => Err(Error::InvalidResponseFromDevice),
        };

        Ok((status_byte, data, checksum_byte?))
    }

    pub fn wait_for_pin_data_ready(&mut self) -> Result<(), Error<S, P>> {
        if let Some(drdy) = &self.drdy {
            let mut timeout = 0;
            loop {
                timeout += 1;
                if drdy.is_low().map_err(Error::PinError)? {
                    return Ok(());
                }

                if timeout > 40000 {
                    return Err(Error::DataReadyTimeoutError);
                }
            }
        } else {
            Err(Error::DRDYPinNotConfigured)
        }
    }

    pub fn calibrate_self_offset1(&mut self) -> Result<([u8; 3]), Error<S, P>> {
        self.write_bitfield(PositiveInpmux::FLOAT)?;
        self.write_bitfield(NegativeInpmux::FLOAT)?;
        self.delay.delay_ms(10);
        self.send_command(Command::SFOCAL1)?;
        self.delay.delay_ms(50);
        let ofcal0 = self.read_register(Register::OFCAL0)?;
        let ofcal1 = self.read_register(Register::OFCAL1)?;
        let ofcal2 = self.read_register(Register::OFCAL2)?;
        Ok([ofcal0, ofcal1, ofcal2])
    }

    pub fn load_calibration_offset1(&mut self, ofcal: [u8; 3]) -> Result<(), Error<S, P>> {
        self.write_register(Register::OFCAL0, ofcal[0])?;
        self.write_register(Register::OFCAL1, ofcal[1])?;
        self.write_register(Register::OFCAL2, ofcal[2])?;
        Ok(())
    }
}
