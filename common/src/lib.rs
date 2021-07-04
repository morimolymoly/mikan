#![no_std]

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum PixelFormat {
    RGBResv8BitPerColor,
    BGRResv8BitPerColor,
}

#[derive(Clone, Copy, Debug)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub frame_buffer_size: usize,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
}