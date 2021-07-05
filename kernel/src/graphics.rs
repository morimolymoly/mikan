use common::{PixelFormat, FrameBufferConfig};
use core::ops::AddAssign;
use core::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn write_pixel(fconfig: FrameBufferConfig, x: u32, y: u32, color: PixelColor) {
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

pub fn write_string(fconfig: FrameBufferConfig, x: u32, y: u32, s: &str, color: PixelColor) {
    let mut i = 0;
    for cc in s.chars() {
        write_ascii(fconfig, x + 8 * i as u32, y, cc, color);
        i+=1;
    }
}

pub fn write_ascii(fconfig: FrameBufferConfig, x: u32, y: u32, c: char, color: PixelColor) {
    use crate::font::get_font;
    let font = get_font(c);
    for dy in 0..16 {
        for dx in 0..8 {
            if (font[dy] << dx) & 0x80 == 0x80 {
                write_pixel(fconfig, x + dx, y + dy as u32, color)
            }
        }
    }
}

pub fn draw_rectangle(fconfig: FrameBufferConfig, pos: Vector2D<u32>, size: Vector2D<u32>, c: PixelColor) {
    for dx in 0..size.x {
        write_pixel(fconfig, pos.x + dx, pos.y , c);
        write_pixel(fconfig, pos.x + dx, pos.y + size.y , c);
    }
    for dy in 0..size.y {
        write_pixel(fconfig, pos.x, pos.y + dy , c);
        write_pixel(fconfig, pos.x + size.x, pos.y + dy , c);
    }
}

pub fn fill_rectangle(fconfig: FrameBufferConfig, pos: Vector2D<u32>, size: Vector2D<u32>, c: PixelColor) {
    for dy in 0..size.y {
        for dx in 0..size.x {
            write_pixel(fconfig, pos.x + dx, pos.y + dy, c);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T
}

impl<T> AddAssign for Vector2D<T> 
    where T: Add<Output=T> + Copy + Clone {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}