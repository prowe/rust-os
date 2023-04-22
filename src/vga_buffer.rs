const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
    chars: [[[u8; 2]; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    text_color: Color,
    background_color: Color,
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // printable ASCII byte or newline
            b'\n' => self.new_line(),
            0x20..=0x7e => self.write_printable_byte(byte),
            // not part of printable ASCII range
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

    fn encode_byte_to_buffer_format(&mut self, byte: u8) -> [u8; 2] {
        let colors = (self.background_color as u8) << 4 | (self.text_color as u8);
        return [byte, colors];
    }

    fn new_line(&mut self) { /* TODO */
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        text_color: Color::Yellow,
        background_color: Color::Black,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
