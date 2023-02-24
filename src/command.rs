#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Command {
    NOP = 0x00,
    RESET = 0x06,
    START1 = 0x08,
    STOP1 = 0x0A,
    START2 = 0x0C,
    STOP2 = 0x0E,
    RDATA1 = 0x12,
    RDATA2 = 0x14,
    SYOCAL1 = 0x16,
    SYGCAL1 = 0x17,
    SFOCAL1 = 0x19,
    SYOCAL2 = 0x1B,
    SYGCAL2 = 0x1C,
    SFOCAL2 = 0x1E,
    RREG = 0x20,
    WREG = 0x40,
}
