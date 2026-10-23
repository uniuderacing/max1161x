// Varianti dei chip della famiglia MAX116xx

pub trait Variant {
    const ADDR: u8;
    const CHANNELS: usize;
    /// true if the last AIN pin is shared with REF (MAX11612/13 and MAX11616/17).
    /// When SEL1=1 that pin leaves the scan, so a full CHANNELS-long scan is impossible.
    const SHARED_REF_PIN: bool;
    /// Allowed supply range in volts.
    const VDD_MIN: f32;
    const VDD_MAX: f32;
}

macro_rules! variant {
    ($name:ident, $addr:expr, $ch:expr, $shared:expr, $vmin:expr, $vmax:expr) => {
        pub struct $name;
        impl Variant for $name {
            const ADDR: u8 = $addr;
            const CHANNELS: usize = $ch;
            const SHARED_REF_PIN: bool = $shared;
            const VDD_MIN: f32 = $vmin;
            const VDD_MAX: f32 = $vmax;
        }
    };
}

//        name       addr       ch  shared  vmin vmax
variant!(MAX11612, 0b0110100,  4,  true,   4.5, 5.5);
variant!(MAX11613, 0b0110100,  4,  true,   2.7, 3.6);
// --- MAX11612 / MAX11613 — 4 canali, package 8-pin ---
variant!(MAX11614, 0b0110011,  8,  false,  4.5, 5.5);
variant!(MAX11615, 0b0110011,  8,  false,  2.7, 3.6);
// --- MAX11614 / MAX11615 — 8 canali, package 16-pin ---
variant!(MAX11616, 0b0110101, 12,  true,   4.5, 5.5);
variant!(MAX11617, 0b0110101, 12,  true,   2.7, 3.6);
// --- MAX11616 / MAX11617 — 12 canali, package 16-pin ---