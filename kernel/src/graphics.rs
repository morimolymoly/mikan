use common::{PixelFormat, FrameBufferConfig};

#[derive(Clone, Copy, Debug)]
pub struct PiexelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn write_pixel(fconfig: FrameBufferConfig, x: u32, y: u32, color: PiexelColor) {
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

pub fn write_ascii(fconfig: FrameBufferConfig, x: u32, y: u32, c: char, color: PiexelColor) {
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