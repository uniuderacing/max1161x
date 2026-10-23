#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    I2c(E),
    BufferTooSmall,
    InvalidChannel,
    /// TCA9543A control register did not read back what was written.
    MuxReadback,
    NotInitialised,
    /// Setup not supported by this driver (see `Max116xx::setup_supported`).
    InvalidSetup,
    /// `vdd` outside the supply range of the ADC variant (or NaN).
    InvalidVdd,
}