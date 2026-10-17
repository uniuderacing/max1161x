use embedded_hal::i2c::I2c;
use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel { Ch0 = 0b01, Ch1 = 0b10 }
impl Channel { pub const ALL: [Channel; 2] = [Channel::Ch0, Channel::Ch1]; }

pub struct Tca9543a { addr: u8 }

impl Tca9543a {
    pub const fn new(a1: bool, a0: bool) -> Self {
        Self { addr: 0x70 | ((a1 as u8) << 1) | a0 as u8 }
    }

    /// Enables exactly one channel and verifies it by read-back.
    pub fn select<I: I2c>(&self, i2c: &mut I, ch: Channel) -> Result<(), Error<I::Error>> {
        i2c.write(self.addr, &[ch as u8]).map_err(Error::I2c)?;
        let mut r = [0u8; 1];
        i2c.read(self.addr, &mut r).map_err(Error::I2c)?;
        if r[0] & 0b11 == ch as u8 { Ok(()) } else { Err(Error::MuxReadback) }
    }

    pub fn disable_all<I: I2c>(&self, i2c: &mut I) -> Result<(), Error<I::Error>> {
        i2c.write(self.addr, &[0x00]).map_err(Error::I2c)
    }
}