#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use core::{panic::PanicInfo, u8};
use common::{PixelFormat, FrameBufferConfig};

const kFontA: &'static [u8] = &[
    0b00000000, //
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b01111110, //  ******
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b11100111, // ***  ***
    0b00000000, //
    0b00000000, //
];

#[derive(Clone, Copy, Debug)]
struct PiexelColor {
    r: u8,
    g: u8,
    b: u8,
}

fn write_pixel(fconfig: FrameBufferConfig, x: u32, y: u32, color: PiexelColor) {
    let pixel_pos = fconfig.pixels_per_scan_line * y + x;
    let frame_buffer = unsafe { core::slice::from_raw_parts_mut(fconfig.frame_buffer,  fconfig.frame_buffer_size)};
    let p = &mut frame_buffer[4 * pixel_pos as usize];
    let p = unsafe { core::slice::from_raw_parts_mut(p, 3)};


    match fconfig.pixel_format {
        PixelFormat::RGBResv8BitPerColor => {     
            p[0] = color.r;  
            p[1] = color.g;
            p[2] = color.b;
        },
        PixelFormat::BGRResv8BitPerColor => {
            p[0] = color.b;
            p[1] = color.g;
            p[2] = color.r;
        }
    }
}

fn write_ascii(fconfig: FrameBufferConfig, x: u32, y: u32, c: char, color: PiexelColor) {
    if c != 'A' {
        return;
    }
    for dy in 0..16 {
        for dx in 0..8 {
            if (kFontA[dy] << dx) & 0x80 == 0x80 {
                write_pixel(fconfig, x + dx, y + dy as u32, color)
            }
        }
    }
}

#[no_mangle]
extern "efiapi" fn kernel_main(fconfig: FrameBufferConfig) -> ! {
    for x in 0..fconfig.horizontal_resolution {
        for y in 0..fconfig.vertical_resolution {
            let color = PiexelColor{r: 255, g: 255, b: 255};
            write_pixel(fconfig, x, y, color);
        }
    }

    for x in 0..200 {
        for y in 0..100 {
            let color = PiexelColor{r: 0, g: 255, b: 0};
            write_pixel(fconfig, x, y, color)
        }
    }

    write_ascii(fconfig, 50, 50, 'A', PiexelColor{r: 0, g: 0, b: 0});
    write_ascii(fconfig, 58, 50, 'A', PiexelColor{r: 0, g: 0, b: 0});

    loop{
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
