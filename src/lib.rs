#![no_std]
#![no_main]

use crate::vga::{CharColor, FrameBuffer, VgaColor};
use core::fmt::Write;

mod vga;
// const CHARS_PER_LINE: u8 = 80;
// const MAX_LINES: u8 = 25;

#[unsafe(no_mangle)]
pub extern "C" fn rust_start() -> ! {
    let mut frame_buffer = FrameBuffer::new();
    // let string = [
    //     Char::new(
    //         b'4',
    //         CharColor::from_colors(VgaColor::Black, VgaColor::White),
    //     ),
    //     Char::new(
    //         b'2',
    //         CharColor::from_colors(VgaColor::Black, VgaColor::White),
    //     ),
    //     Char::new(
    //         b'o',
    //         CharColor::from_colors(VgaColor::Black, VgaColor::White),
    //     ),
    //     Char::new(
    //         b'i',
    //         CharColor::from_colors(VgaColor::Black, VgaColor::White),
    //     ),
    //     Char::new(
    //         b'e',
    //         CharColor::from_colors(VgaColor::Black, VgaColor::White),
    //     ),
    // ];

    frame_buffer.set_color(CharColor::from_colors(VgaColor::Yellow, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                           KFC, Kernel Fried Code                               "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::LightGray, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                         made by lguedes and gsaiago                            "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::Cyan, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::LightCyan, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::White, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                              _  _     ___                                      "
    );
    let _ = write!(
        frame_buffer,
        "                             | || |   |__ \\                                     "
    );
    let _ = write!(
        frame_buffer,
        "                             | || |_     ) |                                    "
    );
    let _ = write!(
        frame_buffer,
        "                             |__   _|   / /                                     "
    );
    let _ = write!(
        frame_buffer,
        "                                | |    / /_                                     "
    );
    let _ = write!(
        frame_buffer,
        "                                |_|   |____|                                    "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::LightBlue, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::Blue, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::LightCyan, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::Cyan, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    frame_buffer.set_color(CharColor::from_colors(VgaColor::LightGray, VgaColor::Black));
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    let _ = write!(
        frame_buffer,
        "                                                                                "
    );
    // ATTENTION: we have a very small stack and no guard page
    // let hello = "Hello World!";
    // let color_byte = 0x1f; // white foreground, blue background
    // //
    // for (i, char) in hello.bytes().enumerate() {
    //     // Each VGA character cell is 2 bytes: character byte + color byte
    //     let offset = (i * 2) as u32;
    //     let char_addr = (VGA_BUFFER_ADDRESS + offset) as *mut u8;
    //     let color_addr = (VGA_BUFFER_ADDRESS + offset + 1) as *mut u8;

    //     unsafe {
    //         core::ptr::write_volatile(char_addr, char);
    //         core::ptr::write_volatile(color_addr, color_byte);
    //     }
    // }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
