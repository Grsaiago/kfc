use core::ptr::NonNull;

use crate::vga::color::CharColor;
use crate::vga::column::Column;
use crate::vga::cursor::Cursor;
use crate::vga::row::Row;

#[derive(Default, Debug)]
pub struct FrameBuffer {
    cursor: Cursor,
    current_color: CharColor,
}

impl FrameBuffer {
    pub const BUF_ADDR: NonNull<u8> = NonNull::new(0xB8000 as *mut u8).unwrap();

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

    /// Clear the entire screen by filling it with spaces
    pub fn clear(&mut self) {
        const BUFFER_SIZE: usize = Column::MAX as usize * Row::MAX as usize * 2; // 80 columns × 25 rows × 2 bytes per cell
        let color_byte = u8::from(self.current_color);

        unsafe {
            for i in (0..BUFFER_SIZE).step_by(2) {
                Self::BUF_ADDR.add(i).write_volatile(b' ');
                Self::BUF_ADDR.add(i + 1).write_volatile(color_byte);
            }
        }

        self.cursor.reset();
    }

    fn get_current_cursor_addr(&self) -> NonNull<u8> {
        let offset = ((self.cursor.row_u8() as usize * Cursor::MAX_COLUMN as usize)
            + (self.cursor.column_u8() as usize))
            * 2; // because the vga buffer is in pairs (char, color)
        unsafe { Self::BUF_ADDR.byte_add(offset) }
    }
}

impl core::fmt::Write for FrameBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            let write_addr = self.get_current_cursor_addr();
            unsafe {
                write_addr.write_volatile(byte);
                write_addr
                    .add(1)
                    .write_volatile(u8::from(self.current_color));
            }

            // If we're at the last cell (end of last line) we should write it
            // and then stop (don't advance past the end of the buffer).
            if self.cursor.is_at_line_end() && self.cursor.is_on_last_line() {
                // Update the hardware cursor to the current position and stop writing
                self.cursor.refresh_vga_cursor_position();
                return Ok(());
            }

            self.cursor.advance();
        }
        self.cursor.refresh_vga_cursor_position();
        Ok(())
    }
}
