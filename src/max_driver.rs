use core::marker::PhantomData;
use embedded_hal::i2c::I2c;
use crate::{
    error::Error, max_messages::{config_byte::*, setup_byte::{Clock, SetupByte}}, max_variants::Variant,
};

pub struct Max116xx<V: Variant>(PhantomData<V>);

impl<V: Variant> Max116xx<V> {
    /// build fails for a Variant with a bad channel count.
    const CHANNELS_OK: () = assert!(V::CHANNELS == 4 || V::CHANNELS == 8 || V::CHANNELS == 12);

    pub const fn new() -> Self { let () = Self::CHANNELS_OK; Self(PhantomData) }

    /// `read_raw` needs 
    /// (a) internal clock (SCAN/read sequence below is the internal-clock one)
    /// (b) a full CHANNELS-long scan. If SEL1=1 the shared AIN_/REF pin leaves the scan,
    /// the device returns CHANNELS-1 results and the last slot would wrap to AIN0.
    pub fn setup_supported(setup: &SetupByte) -> bool {
        let full_scan = !(V::SHARED_REF_PIN && setup.ref_select.ref_on_ain_pin());
        full_scan && matches!(setup.clock, Clock::Internal)
    }

    pub fn configure<I: I2c>(&self, i2c: &mut I, setup: SetupByte) -> Result<(), Error<I::Error>> {
        if !Self::setup_supported(&setup) {
            return Err(Error::InvalidSetup); // no bus traffic
        }
        i2c.write(V::ADDR, &[setup.to_byte()]).map_err(Error::I2c)
    }

    /// Scans AIN0..AIN(CHANNELS-1) single-ended; `out[..CHANNELS]` gets 12-bit codes.
    /// Requires the ADC to be in a `setup_supported` state (power-up default qualifies).
    /// `out` is untouched on error.
    pub fn read_raw<I: I2c>(&self, i2c: &mut I, out: &mut [u16]) -> Result<(), Error<I::Error>> {
        if out.len() < V::CHANNELS { return Err(Error::BufferTooSmall); }
        let last = V::CHANNELS
            .checked_sub(1)
            .and_then(ChannelSelect::from_index)
            .ok_or(Error::InvalidChannel)?;
        let cfg = ConfigByte::new(ScanConfig::ScanFromZero, last, InputMode::SingleEnded).to_byte();

        let mut rx = [0u8; 24];
        let n = 2 * V::CHANNELS;
        // Write config, repeated START, read: allowed by the datasheet (write cycle may end
        // with Sr). Internal clock: the ADC stretches SCL (<= 8.3 us/channel) while converting.
        i2c.write_read(V::ADDR, &[cfg], &mut rx[..n]).map_err(Error::I2c)?;

        for (dst, p) in out.iter_mut().zip(rx[..n].chunks_exact(2)) {
            *dst = (u16::from(p[0] & 0x0F) << 8) | u16::from(p[1]);
        }
        Ok(())
    }
}