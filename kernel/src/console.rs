use spin::Mutex;

use crate::graphics::{PixelColor, write_ascii, write_pixel, write_string};
use common::FrameBufferConfig;
use lazy_static::lazy_static;

const ROWS: usize = 25;
const COLUMNS: usize = 80;

/*
lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console {
        
    }
    )
}*/
pub struct Console {
    cursor_row: usize,
    cursor_column: usize,
    buffer: [[char; COLUMNS]; ROWS],
    fg_color: PixelColor,
    bg_color: PixelColor,
    fconfig: FrameBufferConfig,
}

impl Console {
    pub fn new(fconfig: FrameBufferConfig, fg_color: PixelColor, bg_color: PixelColor) -> Console {
        Console {
            cursor_row: 0,
            cursor_column: 0,
            fg_color: fg_color,
            bg_color: bg_color,
            fconfig: fconfig,
            buffer: [[' '; COLUMNS]; ROWS],
        }   
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