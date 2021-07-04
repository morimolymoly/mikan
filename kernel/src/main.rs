#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

mod graphics;
mod font;

use core::{panic::PanicInfo, u8};
use common::{PixelFormat, FrameBufferConfig};
use graphics::{write_ascii, write_pixel, PiexelColor};


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
