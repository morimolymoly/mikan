#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

mod graphics;
mod font;
mod console;
mod mouse;

use core::panic::PanicInfo;
use common::{PixelFormat, FrameBufferConfig};
use graphics::{write_ascii, write_string, write_pixel, PixelColor, fill_rectangle, draw_rectangle, Vector2D};
use console::{CONSOLE};
use mouse::{MOUSE_CURSOR_FONT, MOUSE_HEIGHT, MOUSE_WIDTH};

const kDesktopBGColor: PixelColor = PixelColor {
    r: 45, 
    g: 118,
    b: 237,
};

const kDesktopFGColor: PixelColor = PixelColor {
    r: 255, 
    g: 255,
    b: 255,
};

#[no_mangle]
extern "efiapi" fn kernel_main(fconfig: FrameBufferConfig) -> ! {
    let KFRAME_WIDTH: u32 = fconfig.horizontal_resolution;
    let KFRAME_HEIGHT: u32 = fconfig.vertical_resolution;

    fill_rectangle(
        fconfig, 
        Vector2D {
            x: 0,
            y: 0
        }, 
        Vector2D {
            x: KFRAME_WIDTH,
            y: KFRAME_HEIGHT - 50,
        }, 
        PixelColor{r: 0, g: 0, b: 0}
    );
    fill_rectangle(
        fconfig, 
        Vector2D {
            x: 0,
            y: KFRAME_HEIGHT - 50,
        }, 
        Vector2D {
            x: KFRAME_WIDTH,
            y: 50,
        }, 
        kDesktopBGColor,
    );
    fill_rectangle(
        fconfig, 
        Vector2D {
            x: 0,
            y: KFRAME_HEIGHT - 50,
        }, 
        Vector2D {
            x: KFRAME_WIDTH / 5,
            y: 50,
        }, 
        PixelColor{r: 80, g: 80, b: 80}
    );
    draw_rectangle(
        fconfig, 
        Vector2D {
            x: 10,
            y: KFRAME_HEIGHT - 40,
        }, 
        Vector2D {
            x: 30,
            y: 30,
        }, 
        PixelColor{r: 160, g: 160, b: 160}
    );

    for dy in 0..MOUSE_HEIGHT {
        let mut dx = 0;
        for c in MOUSE_CURSOR_FONT[dy].chars() {
            if c == '@' {
                write_pixel(fconfig, 200 + dx, 100 + dy as u32, PixelColor{r: 0, g: 0, b: 0});
            } else {
                write_pixel(fconfig, 200 + dx, 100 + dy as u32, PixelColor{r: 255, g: 255, b: 255});
            }
            dx += 1;
        }
    }

    CONSOLE.lock().init(
        fconfig, 
        kDesktopFGColor, 
        kDesktopBGColor
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
