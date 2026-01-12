#![no_std]
#![no_main]

const CHARS_PER_LINE: u8 = 80;
const MAX_LINES: u8 = 25;
const VGA_BUFFER_ADDRESS: u32 = 0xB8000;

#[unsafe(no_mangle)]
pub extern "C" fn start() {
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    for (i, &char) in hello.into_iter().enumerate() {
        let write_addr = (VGA_BUFFER_ADDRESS + i as u32) as *mut u8;
        let character = char | (color_byte << 8);
        unsafe {
            core::ptr::write_volatile(write_addr, character);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
