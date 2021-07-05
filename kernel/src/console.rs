use spin::Mutex;

use crate::graphics::{PixelColor, write_ascii, write_pixel, write_string};
use common::FrameBufferConfig;

use lazy_static::lazy_static;
use core::{fmt, ptr::{null, null_mut}};

const ROWS: usize = 25;
const COLUMNS: usize = 80;


lazy_static! {
    pub static ref CONSOLE: Mutex<Console> =  Mutex::new(Console {
        cursor_row: 0,
        cursor_column: 0,
        buffer: [[' '; COLUMNS]; ROWS],
        fg_color: PixelColor{r: 0, g: 0, b: 0},
        bg_color: PixelColor{r: 255, g: 255, b: 255},
        fconfig: FrameBufferConfig {
            frame_buffer: null_mut(),
            frame_buffer_size: 0,
            pixels_per_scan_line: 0,
            horizontal_resolution: 0,
            vertical_resolution: 0,
            pixel_format: common::PixelFormat::BGRResv8BitPerColor,
        },
    });
}

pub struct Console {
    cursor_row: usize,
    cursor_column: usize,
    buffer: [[char; COLUMNS]; ROWS],
    fg_color: PixelColor,
    bg_color: PixelColor,
    fconfig: FrameBufferConfig,
}

impl Console {
    pub fn init(&mut self, fconfig: FrameBufferConfig, fg_color: PixelColor, bg_color: PixelColor) {
        self.fconfig = fconfig;
        self.fg_color = fg_color;
        self.bg_color = bg_color;
    }

    pub fn put_string(&mut self, string: &str) {
        for c in string.chars() {
            if c == '\n' {
                self.new_line();
            }
            write_ascii(
                self.fconfig, 
                8*self.cursor_column as u32, 
                16*self.cursor_row as u32, 
                c, self.fg_color
            );
            self.buffer[self.cursor_row][self.cursor_column] = c;
            self.cursor_column += 1;
        }
    }

    fn new_line(&mut self) {
        self.cursor_column = 0;
        if self.cursor_row < ROWS -1 {
            self.cursor_row += 1;
            return;
        }

        for y in 0..16*ROWS {
            for x in 0..8*COLUMNS {
                write_pixel(self.fconfig, x as u32, y as u32, self.bg_color);
            }
        }

        for row in 0..ROWS-1 {
            let src = self.buffer[row];
            Console::set_row(&src, &mut self.buffer[row]);
            let mut i = 0;
            for c in self.buffer[row] {
                write_ascii(self.fconfig, 0 + 8 * i as u32, row as u32, c, self.fg_color);
                i+=1;
            }
        }
        Console::clear_row(&mut self.buffer[ROWS-1])
    }

    fn set_row(src: &[char; COLUMNS], dst: &mut [char; COLUMNS]) {
        let mut i = 0;
        for d in dst {
            *d = src[i];
            i += 1;
        }
    }

    fn clear_row(dst: &mut [char; COLUMNS]){
        for d in dst {
            *d = ' ';
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_string(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

unsafe impl Sync for Console {}

unsafe impl Send for Console {}