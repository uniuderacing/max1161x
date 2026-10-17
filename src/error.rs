#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    I2c(E),
    BufferTooSmall,
    InvalidChannel,
    /// TCA9543A control register did not read back what was written.
    MuxReadback,
    NotInitialised,
}