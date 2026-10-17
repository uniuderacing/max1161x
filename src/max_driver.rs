use core::marker::PhantomData;
use embedded_hal::i2c::I2c;
use crate::{
    error::Error,
    max_messages::{config_byte::*, setup_byte::SetupByte},
    max_variants::Variant,
};

pub struct Max116xx<V: Variant>(PhantomData<V>);

impl<V: Variant> Max116xx<V> {
    pub const fn new() -> Self { Self(PhantomData) }

    pub fn configure<I: I2c>(&self, i2c: &mut I, setup: SetupByte) -> Result<(), Error<I::Error>> {
        i2c.write(V::ADDR, &[setup.to_byte()]).map_err(Error::I2c)
    }

    /// Scans AIN0..AIN(CHANNELS-1) single-ended; `out[..CHANNELS]` gets 12-bit codes.
    pub fn read_raw<I: I2c>(&self, i2c: &mut I, out: &mut [u16]) -> Result<(), Error<I::Error>> {
        if out.len() < V::CHANNELS { return Err(Error::BufferTooSmall); }
        let last = V::CHANNELS
            .checked_sub(1)
            .and_then(ChannelSelect::from_index)
            .ok_or(Error::InvalidChannel)?;
        let cfg = ConfigByte::new(ScanConfig::ScanFromZero, last, InputMode::SingleEnded).to_byte();

        let mut rx = [0u8; 24];
        let n = 2 * V::CHANNELS;
        // Repeated-start write+read. [verify] against the datasheet read sequence.
        i2c.write_read(V::ADDR, &[cfg], &mut rx[..n]).map_err(Error::I2c)?;

        for (dst, p) in out.iter_mut().zip(rx[..n].chunks_exact(2)) {
            *dst = (u16::from(p[0] & 0x0F) << 8) | u16::from(p[1]);
        }
        Ok(())
    }
}