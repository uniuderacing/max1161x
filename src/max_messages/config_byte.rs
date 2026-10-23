// this message tells how to read

#[derive(Copy, Clone)]
pub enum ScanConfig {
    /// SCAN=00: scansiona da AIN0 fino al canale scelto
    ScanFromZero = 0b00,
    /// SCAN=01:legge il canale scelto 8 volte
    RepeatEight = 0b01,
    /// SCAN=10: scansiona dalla metà superiore fino al canale scelto
    ScanUpperHalf = 0b10,
    /// SCAN=11: legge solo il canale scelto, una volta
    SingleChannel = 0b11,
}

#[derive(Clone, Copy)]
pub enum ChannelSelect {
    Ain0 = 0b0000,
    Ain1 = 0b0001,
    Ain2 = 0b0010,
    Ain3 = 0b0011,
    Ain4 = 0b0100,
    Ain5 = 0b0101,
    Ain6 = 0b0110,
    Ain7 = 0b0111,
    Ain8 = 0b1000,
    Ain9 = 0b1001,
    Ain10 = 0b1010,
    Ain11 = 0b1011,
}

#[derive(Clone, Copy)]
pub enum InputMode {
    SingleEnded = 1,
    Differential = 0,
}

impl ChannelSelect {
    pub const fn from_index(i: usize) -> Option<Self> {
        Some(match i {
            0 => Self::Ain0, 1 => Self::Ain1, 2 => Self::Ain2, 3 => Self::Ain3,
            4 => Self::Ain4, 5 => Self::Ain5, 6 => Self::Ain6, 7 => Self::Ain7,
            8 => Self::Ain8, 9 => Self::Ain9, 10 => Self::Ain10, 11 => Self::Ain11,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy)]
pub struct ConfigByte {
    pub scanconfig: ScanConfig,
    pub channelselect: ChannelSelect,
    pub mode: InputMode,
}

impl ConfigByte {
    pub fn new(scanconfig: ScanConfig, channelselect: ChannelSelect, mode: InputMode) -> Self {
        Self {
            scanconfig,
            channelselect,
            mode,
        }
    }

    // Costruttore con i default del chip al power-up
    pub fn default() -> Self {
        Self {
            scanconfig: ScanConfig::ScanFromZero,
            channelselect: ChannelSelect::Ain0, // following datasheet default: CS[3:0] = 0000 at power-up
            mode: InputMode::SingleEnded,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let reg: u8 = 0 << 7;
        let scan = (self.scanconfig as u8) << 5;
        let cs = (self.channelselect as u8) << 1;
        let sgl = self.mode as u8;

        let message: u8 = reg | scan | cs | sgl;
        return message;
    }
}
