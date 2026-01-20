use crate::vga::color::CharColor;
use crate::vga::cursor::Cursor;

#[derive(Default, Debug)]
pub struct FrameBuffer {
    cursor: Cursor,
    current_color: CharColor,
}

impl FrameBuffer {
    pub const BUF_ADDR: *mut u8 = 0xB8000 as *mut u8;

    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn set_color(&mut self, color: CharColor) {
        self.current_color = color
    }

    #[inline]
    pub fn color(&self) -> CharColor {
        self.current_color
    }

    pub fn write_byte_character(chr: u8) {
        unsafe {
            Self::BUF_ADDR.write_volatile(chr);
            Self::BUF_ADDR.write_volatile(chr);
        }
    }

    fn get_current_cursor_addr(&self) -> *mut u8 {
        let offset = (self.cursor.row_u8() as usize * Cursor::MAX_COLUMN as usize)
            + (self.cursor.column_u8() as usize) * 2; // because the vga buffer is in pairs (char, color)
        unsafe { Self::BUF_ADDR.byte_add(offset) }
    }
}

impl core::fmt::Write for FrameBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if self.cursor.is_at_line_end() && self.cursor.is_on_last_line() {
                return Ok(());
            }
            let write_addr = self.get_current_cursor_addr();
            unsafe {
                write_addr.write_volatile(byte);
                write_addr
                    .add(1)
                    .write_volatile(u8::from(self.current_color));
            }
            self.cursor.advance();
        }
        Ok(())
    }
}
