use crate::vga::cursor::Cursor;
use crate::vga::color::CharColor;

#[derive(Default, Debug)]
pub struct FrameBuffer {
    cursor: Cursor,
    current_color: CharColor,
}

impl FrameBuffer {
    pub const BUF_ADDR: u32 = 0xB8000;

    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_color(&mut self, color: Color)

    // TODO: instead of recieveing a u8, recieve a time vga::Char or create a Char::from(u8)
    // pub fn write_char(char: u8, y: u32, x: u32) -> Result<(), ()> {
    //     if x > 80 {
    //         return Err;
    //     }
    //     let offset = (pos * 2) as u32;
    //     let char_addr = (Self::BUF_ADDR + (y * 80) + x) as *mut u8;
    //     let color_addr = (Self::BUF_ADDR + offset + 1) as *mut u8;

    //     unsafe {
    //         core::ptr::write_volatile(char_addr, char);
    //         core::ptr::write_volatile(color_addr, color_byte);
    //     }
    // }
}
