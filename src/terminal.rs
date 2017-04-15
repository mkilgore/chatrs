
extern crate libc;
extern crate termios;

use std::io::*;

const BACKSPACE: u8 = 8;
const ERASE_CHAR_SEQUENCE: [u8; 3] = [ BACKSPACE,  b' ', BACKSPACE ];

pub struct Terminal {
    old_termios: termios::Termios,
    last_line_len: usize,
}

impl Terminal {
    pub fn init() -> Result<Terminal> {
        let oterm: termios::Termios = try!(termios::Termios::from_fd(libc::STDIN_FILENO));
        let mut nterm: termios::Termios = oterm;

        nterm.c_lflag &= !(termios::ECHO | termios::ICANON | termios::ECHOE);

        try!(termios::tcsetattr(libc::STDIN_FILENO, termios::TCSANOW, &mut nterm));

        Ok(Terminal { old_termios: oterm, last_line_len: 0 })
    }


    pub fn add_char(&mut self, c: u8) {
        let b: [u8; 1] = [ c ];
        stdout().write(&b).ok();
        stdout().flush().ok();

        self.last_line_len += 1;
    }

    pub fn add_bytes(&mut self, b: &[u8]) {
        stdout().write(b).ok();
        stdout().flush().ok();
        self.last_line_len += b.len();
    }

    pub fn del_char(&mut self) {
        stdout().write(&ERASE_CHAR_SEQUENCE).ok();
        stdout().flush().ok();

        self.last_line_len -= 1;
    }

    pub fn del_bytes(&mut self, mut count: usize) {
        if count > self.last_line_len {
            count = self.last_line_len;
        }

        for _ in 0..count {
            stdout().write(&ERASE_CHAR_SEQUENCE).ok();
        }

        stdout().flush().ok();
        self.last_line_len -= count;
    }

    pub fn erase_line(&mut self) {
        for _ in 0..self.last_line_len {
            stdout().write(&ERASE_CHAR_SEQUENCE).ok();
        }

        stdout().flush().ok();
        self.last_line_len = 0;
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        termios::tcsetattr(libc::STDIN_FILENO, termios::TCSANOW, &mut self.old_termios).ok();
    }
}

