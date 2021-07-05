#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

mod graphics;
mod font;
mod console;

use core::panic::PanicInfo;
use common::{PixelFormat, FrameBufferConfig};
use graphics::{write_ascii, write_string, write_pixel, PixelColor};
use console::{CONSOLE};


#[no_mangle]
extern "efiapi" fn kernel_main(fconfig: FrameBufferConfig) -> ! {
    for x in 0..fconfig.horizontal_resolution {
        for y in 0..fconfig.vertical_resolution {
            let color = PixelColor{r: 255, g: 255, b: 255};
            write_pixel(fconfig, x, y, color);
        }
    }

    for x in 0..200 {
        for y in 0..100 {
            let color = PixelColor{r: 0, g: 255, b: 0};
            write_pixel(fconfig, x, y, color)
        }
    }

    CONSOLE.lock().init(
        fconfig, 
        PixelColor{r: 0, g: 0, b: 0}, 
        PixelColor{r: 255, g: 255, b: 255}
    );

    println!("hello, world! {} ", 10);

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
