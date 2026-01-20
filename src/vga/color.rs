use core::convert::TryFrom;

/// Represents the 16 standard VGA colors where each variant maps to its VGA color code (0-15)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VgaColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

impl Default for VgaColor {
    fn default() -> Self {
        VgaColor::White
    }
}

impl From<VgaColor> for u8 {
    fn from(color: VgaColor) -> u8 {
        color as u8
    }
}

impl TryFrom<u8> for VgaColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(VgaColor::Black),
            0x1 => Ok(VgaColor::Blue),
            0x2 => Ok(VgaColor::Green),
            0x3 => Ok(VgaColor::Cyan),
            0x4 => Ok(VgaColor::Red),
            0x5 => Ok(VgaColor::Magenta),
            0x6 => Ok(VgaColor::Brown),
            0x7 => Ok(VgaColor::LightGray),
            0x8 => Ok(VgaColor::DarkGray),
            0x9 => Ok(VgaColor::LightBlue),
            0xA => Ok(VgaColor::LightGreen),
            0xB => Ok(VgaColor::LightCyan),
            0xC => Ok(VgaColor::LightRed),
            0xD => Ok(VgaColor::LightMagenta),
            0xE => Ok(VgaColor::Yellow),
            0xF => Ok(VgaColor::White),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharColor(u8);

impl Default for CharColor {
    fn default() -> Self {
        // Default: White on Black
        Self::from_colors(VgaColor::White, VgaColor::Black)
    }
}

impl From<CharColor> for u8 {
    fn from(color: CharColor) -> u8 {
        color.0
    }
}

/// Where the first value is the background and the second one is the foreground
impl From<(VgaColor, VgaColor)> for CharColor {
    fn from(value: (VgaColor, VgaColor)) -> Self {
        Self(((value.1 as u8) << 4) | ((value.0 as u8) & 0x0F))
    }
}

impl CharColor {
    /// Create a new color from a raw byte value
    pub const unsafe fn from_raw_u8(value: u8) -> Self {
        Self(value)
    }

    pub const fn from_colors(background: VgaColor, foreground: VgaColor) -> Self {
        Self(((background as u8) << 4) | ((foreground as u8) & 0x0F))
    }

    const fn u8_to_vga_color(&self, value: u8) -> VgaColor {
        match value {
            0x0 => VgaColor::Black,
            0x1 => VgaColor::Blue,
            0x2 => VgaColor::Green,
            0x3 => VgaColor::Cyan,
            0x4 => VgaColor::Red,
            0x5 => VgaColor::Magenta,
            0x6 => VgaColor::Brown,
            0x7 => VgaColor::LightGray,
            0x8 => VgaColor::DarkGray,
            0x9 => VgaColor::LightBlue,
            0xA => VgaColor::LightGreen,
            0xB => VgaColor::LightCyan,
            0xC => VgaColor::LightRed,
            0xD => VgaColor::LightMagenta,
            0xE => VgaColor::Yellow,
            0xF => VgaColor::White,
            _ => unreachable!(),
        }
    }

    pub const fn background(&self) -> VgaColor {
        self.u8_to_vga_color((self.0 >> 4) & 0x0F)
    }

    pub const fn foreground(&self) -> VgaColor {
        self.u8_to_vga_color(self.0 & 0x0F)
    }

    pub fn set_background(&mut self, background: VgaColor) {
        self.0 = (self.0 & 0x0F) | (((background as u8) & 0x0F) << 4);
    }

    pub fn set_foreground(&mut self, foreground: VgaColor) {
        self.0 = (self.0 & 0xF0) | ((foreground as u8) & 0x0F);
    }

    pub fn set_color(&mut self, foreground: VgaColor, background: VgaColor) {
        self.0 = ((background as u8) << 4) | ((foreground as u8) & 0x0F);
    }
}
