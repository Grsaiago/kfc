#![no_std]
#![no_main]

mod vga;

// const CHARS_PER_LINE: u8 = 80;
// const MAX_LINES: u8 = 25;
const VGA_BUFFER_ADDRESS: u32 = 0xB8000;

#[unsafe(no_mangle)]
pub extern "C" fn start() {
    // ATTENTION: we have a very small stack and no guard page
    let hello = "Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    for (i, char) in hello.bytes().enumerate() {
        // Each VGA character cell is 2 bytes: character byte + color byte
        let offset = (i * 2) as u32;
        let char_addr = (VGA_BUFFER_ADDRESS + offset) as *mut u8;
        let color_addr = (VGA_BUFFER_ADDRESS + offset + 1) as *mut u8;

        unsafe {
            core::ptr::write_volatile(char_addr, char);
            core::ptr::write_volatile(color_addr, color_byte);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
