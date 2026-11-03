use embedded_hal::i2c::I2c;
use crate::{
    error::Error, max_driver::Max116xx, max_messages::setup_byte::{RefSelect, SetupByte}, max_variants::Variant, ntc::{Reading, SensorFault, raw_to_celsius, raw_to_millivolts}, tca9543a::{Channel, Tca9543a},
};

pub struct MuxGovernor<I2C: I2c, V: Variant> {
    i2c: I2C,
    mux: Tca9543a,
    adc: Max116xx<V>,
    setup: SetupByte,
    initialised: bool,
}

impl<I2C: I2c, V: Variant> MuxGovernor<I2C, V> {
    pub const SENSORS: usize = 2 * V::CHANNELS;

    /// Rejects setups the driver cannot read correctly, or that `ntc::raw_to_celsius`
    /// cannot convert (it assumes VDD as reference: LSB = vdd/4096). Consumes the bus.
    pub fn new(i2c: I2C, mux: Tca9543a, setup: SetupByte) -> Result<Self, Error<I2C::Error>> {
        if !Max116xx::<V>::setup_supported(&setup)
            || !matches!(setup.ref_select, RefSelect::VddRef)
        {
            return Err(Error::InvalidSetup);
        }
        Ok(Self { i2c, mux, adc: Max116xx::new(), setup, initialised: false })
    }

    pub fn is_initialised(&self) -> bool { self.initialised }
    /// Force re-configuration of both ADCs (e.g. after a bus error or ADC brown-out).
    pub fn invalidate(&mut self) { self.initialised = false; }

    fn configure_all(&mut self) -> Result<(), Error<I2C::Error>> {
        for ch in Channel::ALL {
            self.mux.select(&mut self.i2c, ch)?;
            self.adc.configure(&mut self.i2c, self.setup)?;
        }
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error<I2C::Error>> {
        self.initialised = false;
        self.configure_all()?;
        self.mux.disable_all(&mut self.i2c)?; // always attempted
        self.initialised = true;
        Ok(())
    }

    /// Fills `out[..SENSORS]` (ADC0 channels first, then ADC1).
    /// A failing ADC marks only its own 12 sensors as `Bus`.
    /// Returns the number of ADCs that failed: **callers must check it**; on > 0
    /// call `invalidate()` + `init()`.
    pub fn read_all(&mut self, out: &mut [Reading], vdd: f32, celsius: bool) -> Result<usize, Error<I2C::Error>> {
        if !self.initialised { return Err(Error::NotInitialised); }
        if out.len() < Self::SENSORS { return Err(Error::BufferTooSmall); }
        if !(vdd >= V::VDD_MIN && vdd <= V::VDD_MAX) { return Err(Error::InvalidVdd); }

        let mut failed = 0;
        for (idx, ch) in Channel::ALL.into_iter().enumerate() {
            let slot = &mut out[idx * V::CHANNELS..(idx + 1) * V::CHANNELS];
            let mut raw = [0u16; 12];

            let res = match self.mux.select(&mut self.i2c, ch) {
                Ok(()) => self.adc.read_raw(&mut self.i2c, &mut raw),
                Err(e) => Err(e),
            };
            match res {
                Ok(()) => {
                    for (dst, r) in slot.iter_mut().zip(raw) { 
                        if celsius{
                            *dst = raw_to_celsius(r, vdd); 
                        } else {
                            *dst = raw_to_millivolts(r, vdd); 
                        }
                    }
                }
                Err(_) => { slot.fill(Err(SensorFault::Bus)); failed += 1; }
            }
        }
        let _ = self.mux.disable_all(&mut self.i2c); // best effort
        Ok(failed)
    }
}