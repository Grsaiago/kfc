use core::str;

use super::color::{CharColor, VgaColor};
// use core::ascii::Char;  // queria que a gente usasse isso aqui mas tá unstable, topa usar? -- Claro!
// Mas acho que a gente poderia fazer o

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Char {
    chr: u8,
    color: CharColor,
}

impl Char {
    pub const fn new(chr: u8, color: CharColor) -> Self {
        Self { chr, color }
    }

    pub const fn with_colors(chr: u8, foreground: VgaColor, background: VgaColor) -> Self {
        Self {
            chr,
            color: CharColor::from_colors(foreground, background),
        }
    }

    pub const fn with_foreground(chr: u8, foreground: VgaColor) -> Self {
        Self::with_colors(chr, foreground, VgaColor::Black)
    }

    pub const fn ascii(&self) -> u8 {
        self.chr
    }

    pub const fn color(&self) -> CharColor {
        self.color
    }

    pub fn set_ascii(&mut self, chr: u8) {
        self.chr = chr;
    }

    pub fn set_color(&mut self, color: CharColor) {
        self.color = color;
    }

    pub fn foreground_color(&self) -> VgaColor {
        self.color.foreground()
    }

    pub fn background_color(&self) -> VgaColor {
        self.color.background()
    }

    pub fn set_foreground_color(&mut self, foreground: VgaColor) {
        self.color.set_foreground(foreground);
    }

    pub fn set_background_color(&mut self, background: VgaColor) {
        self.color.set_background(background);
    }

    pub fn set_bg_fg_colors(&mut self, foreground: VgaColor, background: VgaColor) {
        self.color.set_color(foreground, background);
    }
}

/// Extension trait for slices of Char, because we can't `impl [Char]`
/// directly since a `[T]` is a primitive type and we can't add impl blocks to primitive types
pub trait CharSliceExt {
    /// Reinterprets a slice of Chars as a byte slice for direct VGA buffer writing.
    ///
    /// ## SAFETY
    /// - Char is repr(C) with chr (u8) followed by color (CharColor/u8)
    /// - VGA buffer expects: [ascii_byte, color_byte, ascii_byte, color_byte, ...]
    /// - Each Char is 2 bytes laid out as [chr, color] which matches VGA format
    fn as_vga_bytes(&self) -> &[u8];

    /// Converts a slice of Chars to &str in VGA format (ascii, color, ascii, color, ...)
    ///
    /// ## SAFETY
    /// This uses from_utf8_unchecked because VGA bytes aren't valid UTF-8.
    /// Use this ONLY when you need to pass to APIs that expect &str but will
    /// write the raw bytes (like to VGA buffer).
    fn as_vga_str(&self) -> &str;
}

impl CharSliceExt for [Char] {
    fn as_vga_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * core::mem::size_of::<Char>(),
            )
        }
    }

    fn as_vga_str(&self) -> &str {
        let bytes = self.as_vga_bytes();
        unsafe { core::str::from_utf8_unchecked(bytes) }
    }
}

impl Default for Char {
    fn default() -> Self {
        // Default: space character with white on black
        Self::with_colors(b' ', VgaColor::White, VgaColor::Black)
    }
}
