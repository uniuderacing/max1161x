use max1161x::driver::MAX116xx;
use max1161x::devices::variants::{Variant, MAX11615};

fn main() {
    println!("canali: {}", MAX11615::CHANNELS);
    println!("addr: {:#b}", MAX11615::ADDR);
}