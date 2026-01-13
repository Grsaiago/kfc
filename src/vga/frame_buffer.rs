#[derive(Default, Debug)]
pub struct FrameBuffer {}

impl FrameBuffer {
    pub const BUF_ADDR: u32 = 0xB8000;
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write_char(y: u32, x: u32) {
        let offset = (pos * 2) as u32;
        let char_addr = (VGA_BUFFER_ADDRESS + offset) as *mut u8;
        let color_addr = (VGA_BUFFER_ADDRESS + offset + 1) as *mut u8;

        unsafe {
            core::ptr::write_volatile(char_addr, char);
            core::ptr::write_volatile(color_addr, color_byte);
        }
    }
}
