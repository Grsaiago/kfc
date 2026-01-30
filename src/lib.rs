#![no_std]
#![no_main]

use crate::vga::{CharColor, FrameBuffer, VgaColor};
use core::fmt::Write;

mod vga;
mod x86;

#[unsafe(no_mangle)]
pub extern "C" fn rust_start() -> ! {
    let mut frame_buffer = FrameBuffer::new();

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
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
