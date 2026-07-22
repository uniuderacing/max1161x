// Varianti dei chip della famiglia MAX116xx

pub trait Variant {
    const ADDR: u8;
    const CHANNELS: usize;
}

// --- MAX11612 / MAX11613 — 4 canali, package 8-pin ---
pub struct MAX11612;
pub struct MAX11613;

impl Variant for MAX11612 {
    const ADDR: u8 = 0b0110100;
    const CHANNELS: usize = 4;
}

impl Variant for MAX11613 {
    const ADDR: u8 = 0b0110100;
    const CHANNELS: usize = 4;
}

// --- MAX11614 / MAX11615 — 8 canali, package 16-pin ---
pub struct MAX11614;
pub struct MAX11615;

impl Variant for MAX11614 {
    const ADDR: u8 = 0b0110011;
    const CHANNELS: usize = 8;
}

impl Variant for MAX11615 {
    const ADDR: u8 = 0b0110011;
    const CHANNELS: usize = 8;
}

// --- MAX11616 / MAX11617 — 12 canali, package 16-pin ---
pub struct MAX11616;
pub struct MAX11617;

impl Variant for MAX11616 {
    const ADDR: u8 = 0b0110101;
    const CHANNELS: usize = 12;
}

impl Variant for MAX11617 {
    const ADDR: u8 = 0b0110101;
    const CHANNELS: usize = 12;
}


