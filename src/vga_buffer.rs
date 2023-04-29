use core::fmt::{self};
use lazy_static::lazy_static;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const PAGE_437_START: char = 0x20 as char;
const PAGE_437_END: char = 0x7e as char;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        text_color: Color::White,
        background_color: Color::Black,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// pub fn initialize() {
//     let writer = WRITER.lock();
//     lazy_static::initialize(&writer);
// }

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
struct Buffer {
    chars: [[u16; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    text_color: Color,
    background_color: Color,
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_char(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            PAGE_437_START..=PAGE_437_END => self.write_printable_byte(char as u8),
            _ => self.write_printable_byte(0xfe),
        }
    }

    fn write_printable_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;
        self.buffer.chars[row][col] = self.encode_byte_to_buffer_format(byte);
        self.column_position += 1;
    }

    fn encode_byte_to_buffer_format(&mut self, byte: u8) -> u16 {
        let merged =
            (byte as u16) << 8 | (self.background_color as u16) << 4 | (self.text_color as u16);
        return merged.to_be();
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col] = self.buffer.chars[row][col];
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = self.encode_byte_to_buffer_format(' ' as u8);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod tests {
    
}